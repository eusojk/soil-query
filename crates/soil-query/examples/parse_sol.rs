//! Example: Parse a .SOL file and display profile information

use soil_query::SoilProfile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the GI.SOL test file
    let content = std::fs::read_to_string("crates/soil-query/tests/data/GI.SOL")?;
    
    // Parse the .SOL file
    let profiles = SoilProfile::from_sol_format(&content)?;
    
    println!("Found {} profile(s)\n", profiles.len());
    
    // Display information about each profile
    for profile in &profiles {
        println!("Profile ID: {}", profile.id);
        println!("Country: {} ({})", 
            profile.location.country_code, 
            profile.site.country_code_alpha3
        );
        println!("Location: lat={:.3}, lon={:.3}", 
            profile.location.lat, 
            profile.location.lon
        );
        println!("Texture: {}", profile.site.texture);
        println!("Max Depth: {} cm", profile.site.max_depth_cm);
        println!("Layers: {}", profile.layers.len());
        
        // Show first layer details
        if let Some(first_layer) = profile.layers.first() {
            println!("\nFirst Layer (depth={} cm):", first_layer.slb);
            println!("  Wilting Point (SLLL): {:?}", first_layer.slll);
            println!("  Field Capacity (SDUL): {:?}", first_layer.sdul);
            println!("  Saturation (SSAT): {:?}", first_layer.ssat);
            println!("  Bulk Density (SBDM): {:?}", first_layer.sbdm);
            println!("  Organic Carbon (SLOC): {:?}", first_layer.sloc);
            println!("  pH (SLHW): {:?}", first_layer.slhw);
        }
        
        println!("\n{}", "=".repeat(60));
    }
    
    // Demonstrate serialization
    println!("\nSerialized .SOL format:\n");
    let sol_output = profiles[0].to_sol_format();
    println!("{}", sol_output);
    
    Ok(())
}