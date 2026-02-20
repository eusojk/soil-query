# soil-query

> Global soil data with DSSAT-compatible outputs

Query soil profiles for 225 countries at 10km resolution. Built for the DSSAT crop modeling community.


## Quick Start

### Use the API
```bash
# Start the API server
cargo run --release --bin soil-query-api

# Query soil data for any location
curl "http://127.0.0.1:3000/soil?lat=42.7&lon=-84.5&format=json"

# Get .SOL format output
curl "http://127.0.0.1:3000/soil?lat=42.7&lon=-84.5&format=sol"

# Check server health
curl http://127.0.0.1:3000/health
```

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

### Parse .SOL Files into Database
```bash
# Parse all .SOL files from a directory
cargo run --release --bin soil-query-parser -- \
    --input /path/to/sol_files \
    --output ./output/soil_data.db \
    --report ./output/report.json \
    --verbose
```

**Example output:**
```
soil-query-parser v0.1.0
Found 225 .SOL files

Processing: US.SOL
  Parsed 161724 profiles from US.SOL
  Progress: 161724/161724 (100%)
✓ US.SOL: 161724 profiles

...

============================================================
PARSING COMPLETE
============================================================
Files processed:    225
Profiles parsed:    1984797
Successful:         1984797
Failed:             0
Duration:           583.90s
Profiles/second:    3399
Database size:      4102.95 MB
============================================================
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


## API Endpoints

### GET /health

Check server status and profile count.
```bash
curl http://127.0.0.1:3000/health
```

**Response:**
```json
{
  "status": "ok",
  "version": "0.1.0",
  "profiles": 1984797
}
```

### GET /soil

Query soil data by coordinates.

**Parameters:**
- `lat` (required): Latitude (-90 to 90)
- `lon` (required): Longitude (-180 to 180)  
- `format` (optional): "json" (default) or "sol"

**Example:**
```bash
curl "http://127.0.0.1:3000/soil?lat=42.7&lon=-84.5&format=json"
```

**Response:**
```json
{
  "profile": {
    "id": "US02450585",
    "location": {"lat": 42.708, "lon": -84.542, "country_code": "US"},
    "site": {"texture": "Loam", "max_depth_cm": 200},
    "layers": [...]
  },
  "distance_km": 3.545
}
```

### GET /definitions

Get property definitions for soil abbreviations.
```bash
curl http://127.0.0.1:3000/definitions
```

## CLI Tool

### Installation
```bash
# Build from source
cargo build --release --bin soil-query

# The binary is at: target/release/soil-query (or .exe on Windows)
```

### Quick Start
```bash
# Find soil data for any location
soil-query find --lat=42.7 --lon=-84.5

# Get JSON output
soil-query find --lat=42.7 --lon=-84.5 --format=json

# Save as .SOL file
soil-query find --lat=42.7 --lon=-84.5 --format=sol --output=soil.SOL

# List property definitions
soil-query definitions

# Explain a property
soil-query explain SLLL

# Show database statistics
soil-query stats
```

### CLI Commands

#### find - Query Soil Data

Find the nearest soil profile for any coordinates.
```bash
soil-query find --lat=<latitude> --lon=<longitude> [OPTIONS]
```

**Options:**
- `--lat, -l`: Latitude (-90 to 90)
- `--lon, -n`: Longitude (-180 to 180)
- `--format, -f`: Output format (summary/json/sol)
- `--output, -o`: Save to file

**Example output:**
```
→ Searching for soil data...
  Location: 42.700°, -84.500°
✓ Found profile!
  ID: US02450585
  Distance: 3.55 km

Profile Summary

  ID: US02450585
  Country: US
  Location: 42.708°, -84.542°
  Texture: Loam
  Max Depth: 200 cm

Soil Layers

  Depth    Texture  WP       FC       SAT      pH
  5        A        0.101    0.227    0.389    6.250
  15       A        0.112    0.237    0.391    6.320
  ...
```

#### definitions - List Properties

List all soil property abbreviations with descriptions.
```bash
soil-query definitions
```

#### explain - Property Details

Get detailed explanation for a specific property.
```bash
soil-query explain <ABBREVIATION>
```

**Examples:**
```bash
soil-query explain SLLL    # Wilting point
soil-query explain SBDM    # Bulk density  
soil-query explain SLHW    # pH in water
```

#### stats - Database Statistics

Show database statistics and top countries.
```bash
soil-query stats
```

**Output:**
```
Database Statistics

  Total Profiles: 1984797

  Top 10 Countries:
    RU 421983 profiles (21.3%)
    CA 244237 profiles (12.3%)
    US 161724 profiles (8.1%)
    ...

  Database Size: 4102.95 MB
```

### Known Limitations

1. **Database required**: Must have `soil_data.db` created or downloaded
2. **No caching**: Each query hits the database
3. **Limited definitions**: Only 9 properties included at the moment (expansion is a TODO item)
4. **No batch queries**: One location at a time (by design for simplicity)


## Project Structure
```
soil-query/
├── crates/
│   ├── soil-query/              # Core library
│   │   ├── src/
│   │   │   ├── lib.rs           # Public API
│   │   │   ├── types.rs         # Data structures
│   │   │   ├── parser.rs        # .SOL parser & serializer
│   │   │   ├── error.rs         # Error types
│   │   │   └── definitions.rs   # Property definitions
│   │   ├── tests/
│   │   │   ├── parser_tests.rs  # Integration tests
│   │   │   └── data/            # Test .SOL files
│   │   └── examples/
│   │       └── parse_sol.rs     # Usage example
│   │
│   ├── soil-query-parser/       # Data parser
│   │   ├── src/
│   │   │   ├── main.rs          # CLI application
│   │   │   ├── db.rs            # Database operations
│   │   │   ├── validation.rs    # Profile validation
│   │   │   └── report.rs        # Statistics generation
│   │   └── examples/
│   │       ├── inspect_files.rs # Inspect .SOL files
│   │       └── show_ids.rs      # Show profile IDs
│   │
│   ├── soil-query-api/          # REST API
│   │   ├── src/
│   │   │   ├── main.rs          # Server entry point
│   │   │   ├── db.rs            # Database queries
│   │   │   ├── handlers.rs      # Request handlers
│   │   │   └── models.rs        # API types
│   │   └── README.md            # API documentation
│   │
│   └── soil-query-cli/          # CLI tool
│       ├── src/
│       │   └── main.rs          # CLI application
│       └── README.md            # CLI documentation
│
├── test_data/                   # 10 test .SOL files
├── output/                      # Generated files
│   ├── soil_data.db             # Production database (4.1 GB)
│   ├── test.db                  # Test database
│   ├── full_parse_report.json   # Parse statistics
│   └── DATABASE_INFO.md         # Database documentation
└── README.md
```

## Features

### Production Database

**Coverage:**
- **225 countries** worldwide
- **1,984,797 soil profiles**
- **~10 km resolution** (5 arc-minute)
- **6 standard depths**: 5, 15, 30, 60, 100, 200 cm

**Top Countries:**
| Country | Profiles | % of Total |
|---------|----------|------------|
| Russia | 421,983 | 21.3% |
| Canada | 244,237 | 12.3% |
| USA | 161,724 | 8.1% |
| China | 120,107 | 6.1% |
| Brazil | 102,731 | 5.2% |
| Australia | 102,180 | 5.1% |

**Database Schema:**
```sql
-- Main table with profile data
CREATE TABLE soil_profiles (
    rowid INTEGER PRIMARY KEY AUTOINCREMENT,
    id TEXT NOT NULL UNIQUE,
    country_code TEXT NOT NULL,
    lat REAL NOT NULL,
    lon REAL NOT NULL,
    ...
);

-- Spatial index for fast coordinate queries
CREATE VIRTUAL TABLE soil_spatial_idx USING rtree(
    rowid,
    min_lat, max_lat,
    min_lon, max_lon
);
```

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

# Run parser tests
cargo test --package soil-query-parser

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_parse_single_profile_gi

# Check code quality
cargo clippy --all-targets --all-features

# Test with small dataset
cargo run --bin soil-query-parser -- \
    --input ./test_data \
    --output ./output/test.db \
    --verbose

# View documentation
cargo doc --open
```

### Performance Benchmarks

| Dataset | Files | Profiles | Time | Speed |
|---------|-------|----------|------|-------|
| Test | 10 | 161,746 | 32.75s | 4,939/s |
| Production | 225 | 1,984,797 | 583.90s | 3,399/s |


### Test Coverage
TODO


## Contributing
TODO


## License
TODO

## Data Source
TODO
