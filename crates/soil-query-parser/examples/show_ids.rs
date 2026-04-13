//! Show profile IDs in each file

use soil_query::SoilProfile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for file in ["GI.SOL", "MC.SOL"] {
        let path = format!("test_data/{}", file);
        let content = std::fs::read_to_string(&path)?;
        let profiles = SoilProfile::from_sol_format(&content)?;

        println!("\n{}:", file);
        for profile in profiles {
            println!("  - {}", profile.id);
        }
    }

    Ok(())
}
