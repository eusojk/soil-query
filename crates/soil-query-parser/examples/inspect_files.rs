//! Inspect .SOL files for duplicate IDs

use soil_query::SoilProfile;
use std::collections::HashMap;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_dir = "test_data";
    
    let mut all_ids: HashMap<String, Vec<String>> = HashMap::new();
    
    // Find all .SOL files
    let sol_files: Vec<_> = WalkDir::new(test_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("SOL"))
                .unwrap_or(false)
        })
        .collect();
    
    println!("Inspecting {} files...\n", sol_files.len());
    
    for entry in sol_files {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        
        let content = std::fs::read_to_string(path)?;
        let profiles = SoilProfile::from_sol_format(&content)?;
        
        println!("{}: {} profiles", filename, profiles.len());
        
        for profile in profiles {
            all_ids.entry(profile.id.clone())
                .or_insert_with(Vec::new)
                .push(filename.to_string());
        }
    }
    
    // Find duplicates
    println!("\nChecking for duplicates...");
    let mut has_duplicates = false;
    
    for (id, files) in &all_ids {
        if files.len() > 1 {
            println!("  Duplicate ID '{}' found in: {:?}", id, files);
            has_duplicates = true;
        }
    }
    
    if !has_duplicates {
        println!("  No duplicates found!");
    }
    
    println!("\nTotal unique profiles: {}", all_ids.len());
    
    Ok(())
}

