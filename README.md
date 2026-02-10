# soil-query

> Global soil data with DSSAT-compatible outputs

Query soil profiles for 225 countries at 10km resolution. Built for the DSSAT crop modeling community.


## Quick Start

### As a Library
```rust
use soil_query::SoilProfile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read a .SOL file
    let content = std::fs::read_to_string("GI.SOL")?;
    
    // Parse it
    let profiles = SoilProfile::from_sol_format(&content)?;
    
    // Access data
    for profile in profiles {
        println!("Profile: {}", profile.id);
        println!("Location: {:.3}, {:.3}", 
            profile.location.lat, 
            profile.location.lon
        );
        println!("Layers: {}", profile.layers.len());
        
        // Access layer data
        for layer in &profile.layers {
            println!("  Depth {} cm: pH={:?}, OC={:?}%", 
                layer.slb, 
                layer.slhw,  // pH in water
                layer.sloc   // Organic carbon
            );
        }
    }
    
    // Serialize back to .SOL format
    let sol_output = profiles[0].to_sol_format();
    std::fs::write("output.SOL", sol_output)?;
    
    Ok(())
}
```

### Run the Example
```bash
cargo run --example parse_sol
```

Output:
```
Found 1 profile(s)

Profile ID: GI02792815
Country: GI (GIB)
Location: lat=36.125, lon=-5.375
Texture: Loam
Max Depth: 200 cm
Layers: 6

First Layer (depth=5 cm):
  Wilting Point (SLLL): Some(0.143)
  Field Capacity (SDUL): Some(0.258)
  Saturation (SSAT): Some(0.409)
  ...
```

## Project Structure
```
soil-query/
├── crates/
│   └── soil-query/              # Core library 
│       ├── src/
│       │   ├── lib.rs           # Public API
│       │   ├── types.rs         # Data structures
│       │   ├── parser.rs        # .SOL parser & serializer
│       │   ├── error.rs         # Error types
│       │   └── definitions.rs   # Property definitions
│       ├── tests/
│       │   ├── parser_tests.rs  # Integration tests
│       │   └── data/            # Test .SOL files
│       └── examples/
│           └── parse_sol.rs     # Usage example
└── README.md
```

## Features

### Data Structure

Each soil profile contains:
- **Location**: Coordinates (lat/lon) and country code
- **Site properties**: Texture, depth, soil family
- **Site-wide properties**: Albedo, drainage, runoff, etc.
- **6 soil layers** at depths: 5, 15, 30, 60, 100, 200 cm

Each layer includes ~17 properties:
- Water retention (wilting point, field capacity, saturation)
- Physical properties (bulk density, texture percentages)
- Chemical properties (organic carbon, pH, CEC)

## Development

### Testing
```bash
# Run all tests
cargo test --all

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_parse_single_profile_gi

# Check code quality
cargo clippy --all-targets --all-features

# View documentation
cargo doc --open
```

### Test Coverage
TODO


## Contributing
TODO


## License
TODO

## Data Source
TODO
