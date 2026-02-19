//! soil-query CLI: Command-line tool for querying soil data

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "soil-query")]
#[command(version, about = "Query global soil data with DSSAT-compatible outputs", long_about = None)]
struct Cli {
    /// Path to the SQLite database
    #[arg(short, long, default_value = "output/soil_data.db", global = true)]
    database: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Find soil data for coordinates
    Find {
        /// Latitude in decimal degrees
        #[arg(short, long)]
        lat: f64,

        /// Longitude in decimal degrees
        #[arg(short = 'n', long)]
        lon: f64,

        /// Output format: json, sol, or summary
        #[arg(short, long, default_value = "summary")]
        format: String,

        /// Output file path (prints to stdout if not provided)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// List all property definitions
    Definitions,

    /// Explain a specific property abbreviation
    Explain {
        /// Property abbreviation (e.g., SLLL, SBDM, SLHW)
        abbreviation: String,
    },

    /// Show database statistics
    Stats,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Check if database exists
    if !cli.database.exists() {
        eprintln!(
            "{} Database not found: {:?}",
            "Error:".red().bold(),
            cli.database
        );
        eprintln!("\n{}", "Run soil-query-parser to create the database first.".yellow());
        std::process::exit(1);
    }

    // Execute command
    match cli.command {
        Commands::Find { lat, lon, format, output } => {
            cmd_find(&cli.database, lat, lon, &format, output.as_deref())?;
        }
        Commands::Definitions => {
            cmd_definitions()?;
        }
        Commands::Explain { abbreviation } => {
            cmd_explain(&abbreviation)?;
        }
        Commands::Stats => {
            cmd_stats(&cli.database)?;
        }
    }

    Ok(())
}

/// Find soil data for coordinates
fn cmd_find(
    db_path: &PathBuf,
    lat: f64,
    lon: f64,
    format: &str,
    output_path: Option<&std::path::Path>,
) -> Result<()> {
    // Validate coordinates
    if !(-90.0..=90.0).contains(&lat) {
        anyhow::bail!("Invalid latitude: {} (must be -90 to 90)", lat);
    }
    if !(-180.0..=180.0).contains(&lon) {
        anyhow::bail!("Invalid longitude: {} (must be -180 to 180)", lon);
    }

    println!("{} Searching for soil data...", "→".cyan());
    println!("  Location: {:.3}°, {:.3}°", lat, lon);

    // Open database
    let conn = rusqlite::Connection::open(db_path)
        .context("Failed to open database")?;

    // Find nearest profile (using same logic as API)
    let (profile, distance) = find_nearest_profile(&conn, lat, lon)?;

    println!("{} Found profile!", "✓".green().bold());
    println!("  ID: {}", profile.id.bright_white());
    println!("  Location: {:.3}°, {:.3}°", profile.location.lat, profile.location.lon);
    println!("  Distance: {:.2} km", distance);
    println!();

    // Output based on format
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&profile)?;
            if let Some(path) = output_path {
                std::fs::write(path, json)?;
                println!("{} Saved to: {:?}", "✓".green(), path);
            } else {
                println!("{}", json);
            }
        }
        "sol" => {
            let sol_content = profile.to_sol_format();
            if let Some(path) = output_path {
                std::fs::write(path, sol_content)?;
                println!("{} Saved to: {:?}", "✓".green(), path);
            } else {
                println!("{}", sol_content);
            }
        }
        "summary" | _ => {
            print_profile_summary(&profile, distance);
        }
    }

    Ok(())
}

/// Print a human-readable summary of the profile
fn print_profile_summary(profile: &soil_query::SoilProfile, distance: f64) {
    println!("{}", "Profile Summary".bold().underline());
    println!();
    println!("  {}: {}", "ID".bright_cyan(), profile.id);
    println!("  {}: {}", "Country".bright_cyan(), profile.location.country_code);
    println!("  {}: {:.3}°, {:.3}°", "Location".bright_cyan(), profile.location.lat, profile.location.lon);
    println!("  {}: {:.2} km", "Distance".bright_cyan(), distance);
    println!("  {}: {}", "Texture".bright_cyan(), profile.site.texture);
    println!("  {}: {} cm", "Max Depth".bright_cyan(), profile.site.max_depth_cm);
    println!("  {}: {}", "Source".bright_cyan(), profile.metadata.source);
    println!();

    println!("{}", "Soil Layers".bold().underline());
    println!();
    println!("  {:<8} {:<8} {:<8} {:<8} {:<8} {:<8}",
        "Depth", "Texture", "WP", "FC", "SAT", "pH");
    println!("  {:<8} {:<8} {:<8} {:<8} {:<8} {:<8}",
        "(cm)", "", "(cm³/cm³)", "(cm³/cm³)", "(cm³/cm³)", "");
    println!("  {}", "─".repeat(60));

    for layer in &profile.layers {
        println!("  {:<8} {:<8} {:<8} {:<8} {:<8} {:<8}",
            layer.slb,
            layer.slmh,
            format_optional(layer.slll),
            format_optional(layer.sdul),
            format_optional(layer.ssat),
            format_optional(layer.slhw),
        );
    }
    println!();
    println!("{}", "Legend:".italic());
    println!("  WP = Wilting Point (SLLL)");
    println!("  FC = Field Capacity (SDUL)");
    println!("  SAT = Saturation (SSAT)");
}

/// Format optional value for display
fn format_optional(value: Option<f64>) -> String {
    match value {
        Some(v) => format!("{:.3}", v),
        None => "-".to_string(),
    }
}

/// List all property definitions
fn cmd_definitions() -> Result<()> {
    println!("{}", "Soil Property Definitions".bold().underline());
    println!();

    // TODO: Load from definitions module
    let definitions = vec![
        ("SLLL", "Lower limit (wilting point)", "cm³/cm³"),
        ("SDUL", "Drained upper limit (field capacity)", "cm³/cm³"),
        ("SSAT", "Upper limit, saturated", "cm³/cm³"),
        ("SBDM", "Bulk density", "g/cm³"),
        ("SLOC", "Organic carbon", "%"),
        ("SLCL", "Clay (<0.002 mm)", "%"),
        ("SLSI", "Silt (0.05 to 0.002 mm)", "%"),
        ("SLHW", "pH in water", ""),
        ("SCEC", "Cation exchange capacity", "cmol/kg"),
    ];

    for (abbrev, name, unit) in definitions {
        println!("  {} - {}", abbrev.bright_cyan().bold(), name);
        if !unit.is_empty() {
            println!("      Unit: {}", unit.italic());
        }
        println!();
    }

    Ok(())
}

/// Explain a specific property
fn cmd_explain(abbreviation: &str) -> Result<()> {
    let abbrev_upper = abbreviation.to_uppercase();

    // TODO: Load from definitions module
    let explanation = match abbrev_upper.as_str() {
        "SLLL" => Some((
            "Lower limit (wilting point)",
            "cm³/cm³",
            "Volumetric water content at which plants can no longer extract water (permanent wilting point)"
        )),
        "SDUL" => Some((
            "Drained upper limit (field capacity)",
            "cm³/cm³",
            "Volumetric water content after excess water has drained by gravity"
        )),
        "SSAT" => Some((
            "Upper limit, saturated",
            "cm³/cm³",
            "Volumetric water content when all pore space is filled with water"
        )),
        "SBDM" => Some((
            "Bulk density",
            "g/cm³",
            "Mass of dry soil per unit volume"
        )),
        "SLOC" => Some((
            "Organic carbon",
            "%",
            "Percentage of organic carbon in the soil"
        )),
        "SLHW" => Some((
            "pH in water",
            "",
            "Soil pH measured in water solution"
        )),
        _ => None,
    };

    match explanation {
        Some((name, unit, description)) => {
            println!("{}", abbrev_upper.bright_cyan().bold().underline());
            println!();
            println!("  {}: {}", "Full Name".bold(), name);
            if !unit.is_empty() {
                println!("  {}: {}", "Unit".bold(), unit);
            }
            println!();
            println!("  {}", description);
            println!();
        }
        None => {
            println!("{} Unknown property: {}", "Error:".red().bold(), abbreviation);
            println!();
            println!("Run {} to see all available properties.", "soil-query definitions".yellow());
        }
    }

    Ok(())
}

/// Show database statistics
fn cmd_stats(db_path: &PathBuf) -> Result<()> {
    let conn = rusqlite::Connection::open(db_path)?;

    println!("{}", "Database Statistics".bold().underline());
    println!();

    // Total profiles
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM soil_profiles", [], |row| row.get(0))?;
    println!("  {}: {}", "Total Profiles".bright_cyan(), total.to_string().bright_white());

    // Profiles by country (top 10)
    let mut stmt = conn.prepare(
        "SELECT country_code, COUNT(*) as count 
         FROM soil_profiles 
         GROUP BY country_code 
         ORDER BY count DESC 
         LIMIT 10"
    )?;

    let countries: Vec<(String, i64)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;

    println!();
    println!("  {}", "Top 10 Countries:".bold());
    for (country, count) in countries {
        let percentage = (count as f64 / total as f64) * 100.0;
        println!("    {} {} profiles ({:.1}%)",
            country.bright_yellow(),
            count.to_string().bright_white(),
            percentage
        );
    }

    // Database file size
    let metadata = std::fs::metadata(db_path)?;
    let size_mb = metadata.len() as f64 / 1_048_576.0;
    println!();
    println!("  {}: {:.2} MB", "Database Size".bright_cyan(), size_mb);

    Ok(())
}

/// Find nearest profile (same logic as API)
fn find_nearest_profile(
    conn: &rusqlite::Connection,
    lat: f64,
    lon: f64,
) -> Result<(soil_query::SoilProfile, f64)> {
    let search_radius = 0.5;

    let mut stmt = conn.prepare(
        "SELECT p.data, p.lat, p.lon
         FROM soil_profiles p
         WHERE p.rowid IN (
             SELECT rowid FROM soil_spatial_idx
             WHERE min_lat >= ?1 AND max_lat <= ?2
               AND min_lon >= ?3 AND max_lon <= ?4
         )
         LIMIT 100"
    )?;

    let rows = stmt.query_map(
        [
            lat - search_radius,
            lat + search_radius,
            lon - search_radius,
            lon + search_radius,
        ],
        |row| {
            let data: String = row.get(0)?;
            let profile_lat: f64 = row.get(1)?;
            let profile_lon: f64 = row.get(2)?;
            Ok((data, profile_lat, profile_lon))
        },
    )?;

    let mut nearest: Option<(soil_query::SoilProfile, f64)> = None;

    for row in rows {
        let (data, profile_lat, profile_lon) = row?;
        let mut profile: soil_query::SoilProfile = serde_json::from_str(&data)?;
        let distance = haversine_distance(lat, lon, profile_lat, profile_lon);
        profile.metadata.distance_km = Some(distance);

        match &nearest {
            None => nearest = Some((profile, distance)),
            Some((_, best_distance)) => {
                if distance < *best_distance {
                    nearest = Some((profile, distance));
                }
            }
        }
    }

    nearest.ok_or_else(|| anyhow::anyhow!("No profiles found near coordinates"))
}

/// Haversine distance calculation
fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;

    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_KM * c
}