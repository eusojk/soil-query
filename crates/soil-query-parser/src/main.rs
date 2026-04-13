//! soil-query-parser: Parse .SOL files into SQLite database

mod db;
mod report;
mod validation;

use anyhow::{Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "soil-query-parser")]
#[command(about = "Parse .SOL files into SQLite database with spatial indexing", long_about = None)]
struct Args {
    /// Input directory containing .SOL files
    #[arg(short, long)]
    input: PathBuf,

    /// Output database path
    #[arg(short, long, default_value = "output/soil_data.db")]
    output: PathBuf,

    /// Report JSON output path
    #[arg(short, long, default_value = "output/parse_report.json")]
    report: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("soil-query-parser v{}", env!("CARGO_PKG_VERSION"));
    println!("Input:  {:?}", args.input);
    println!("Output: {:?}", args.output);
    println!();

    // Validate input directory exists
    if !args.input.exists() {
        anyhow::bail!("Input directory does not exist: {:?}", args.input);
    }

    // Start timer
    let start = Instant::now();

    // Initialize database
    println!("Initializing database...");
    let conn = db::init_database(&args.output)?;

    // Initialize report
    let mut report = report::ParseReport::new();

    // Find all .SOL files
    let sol_files: Vec<PathBuf> = WalkDir::new(&args.input)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("SOL"))
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    if sol_files.is_empty() {
        anyhow::bail!("No .SOL files found in input directory");
    }

    println!("Found {} .SOL files\n", sol_files.len());
    report.summary.total_files = sol_files.len();

    // Create progress bar
    let pb = ProgressBar::new(sol_files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})\n{msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Process each file
    for sol_file in &sol_files {
        let filename = sol_file
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        pb.set_message(format!("Processing: {}", filename));

        match process_file(&conn, sol_file, &mut report, args.verbose) {
            Ok(count) => {
                if args.verbose {
                    pb.println(format!("✓ {}: {} profiles", filename, count));
                }
            }
            Err(e) => {
                // Fail-fast: stop on first error
                pb.finish_with_message(format!("✗ Failed on {}: {}", filename, e));
                return Err(e.context(format!("Failed to process file: {}", filename)));
            }
        }

        pb.inc(1);
    }

    pb.finish_with_message("All files processed successfully!");

    // Finalize database
    println!("\nFinalizing database...");
    db::finalize_database(&conn)?;

    // Calculate duration
    let duration = start.elapsed();
    let duration_secs = duration.as_secs_f64();

    // Finalize report
    report.finalize(duration_secs, &args.output)?;

    // Print summary
    report.print_summary();

    // Save report
    report.save(&args.report).context("Failed to save report")?;
    println!("\nReport saved to: {:?}", args.report);

    Ok(())
}

/// Process a single .SOL file
fn process_file(
    conn: &rusqlite::Connection,
    file_path: &PathBuf,
    report: &mut report::ParseReport,
    verbose: bool,
) -> Result<usize> {
    let filename = file_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    // Read file
    let content = std::fs::read_to_string(file_path).context("Failed to read file")?;

    // Parse profiles
    let profiles = soil_query::SoilProfile::from_sol_format(&content)
        .context("Failed to parse .SOL format")?;

    if verbose {
        println!("  Parsed {} profiles from {}", profiles.len(), filename);
        if !profiles.is_empty() {
            println!("  First profile ID: {}", profiles[0].id);
        }
    }

    // Use a transaction for batch inserts (much faster)
    let tx = conn.unchecked_transaction()?;

    // For large files, show progress
    let show_progress = profiles.len() > 10000;
    let progress_interval = if show_progress {
        profiles.len() / 100
    } else {
        usize::MAX
    };

    // Process each profile
    for (idx, profile) in profiles.iter().enumerate() {
        // Show progress for large files
        if show_progress && idx % progress_interval == 0 && idx > 0 {
            print!(
                "\r  Progress: {}/{} ({:.0}%)",
                idx,
                profiles.len(),
                (idx as f64 / profiles.len() as f64) * 100.0
            );
            use std::io::Write;
            std::io::stdout().flush().ok();
        }

        // Validate
        validation::validate_profile(profile)
            .context(format!("Validation failed for profile {}", profile.id))?;

        // Insert
        if let Err(e) = db::insert_profile(&tx, profile) {
            eprintln!("\nError inserting profile #{} (ID: {})", idx, profile.id);
            eprintln!("File: {}", filename);
            eprintln!("Error: {:?}", e);
            return Err(e.context(format!("Failed to insert profile {}", profile.id)));
        }

        // Record success
        report.record_success(&profile.location.country_code);
    }

    if show_progress {
        println!("\r  Progress: {}/{} (100%)", profiles.len(), profiles.len());
    }

    // Commit the transaction
    tx.commit()?;

    if verbose {
        println!("  Committed {} profiles", profiles.len());
    }

    Ok(profiles.len())
}
