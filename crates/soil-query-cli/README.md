# soil-query CLI

Command-line tool for querying global soil data with DSSAT-compatible outputs.

## Installation

### From Source
```bash
cargo build --release --bin soil-query
```

The binary will be at `target/release/soil-query` (or `soil-query.exe` on Windows).

### Copy to PATH (Optional)
```bash
# Linux/Mac
cp target/release/soil-query ~/.local/bin/

# Windows (PowerShell as Administrator)
Copy-Item target\release\soil-query.exe C:\Windows\System32\
```

## Usage

### Find Soil Data

Query soil data for any location on Earth.
```bash
# Basic query (shows summary)
soil-query find --lat=42.7 --lon=-84.5

# JSON output
soil-query find --lat=42.7 --lon=-84.5 --format=json

# .SOL format output
soil-query find --lat=42.7 --lon=-84.5 --format=sol

# Save to file
soil-query find --lat=42.7 --lon=-84.5 --format=sol --output=soil.SOL
```

**Note for Windows PowerShell**: Use `--lat=42.7 --lon=-84.5` format (with equals signs) to avoid issues with negative numbers.

### List Property Definitions
```bash
soil-query definitions
```

Shows all soil property abbreviations with descriptions and units.

### Explain a Property
```bash
soil-query explain SLLL
soil-query explain SBDM
soil-query explain SLHW
```

Get detailed explanation for a specific soil property.

### Database Statistics
```bash
soil-query stats
```

Shows total profiles, top countries, and database size.

## Output Formats

### Summary (Default)

Human-readable table format with key properties:
```
Profile Summary

  ID: US02450585
  Country: US
  Location: 42.708°, -84.542°
  Distance: 3.55 km
  Texture: Loam
  Max Depth: 200 cm

Soil Layers

  Depth    Texture  WP       FC       SAT      pH
  5        A        0.101    0.227    0.389    6.250
  15       A        0.112    0.237    0.391    6.320
  ...
```

### JSON

Machine-readable format with complete profile data:
```json
{
  "id": "US02450585",
  "location": {
    "lat": 42.708,
    "lon": -84.542,
    "country_code": "US"
  },
  "site": {...},
  "layers": [...]
}
```

### .SOL

DSSAT-compatible format ready for crop modeling:
```
*US02450585     USA              Loam   200    ISRIC soilgrids + HC27
@SITE        COUNTRY          LAT     LONG SCS Family
 -99              US      42.708   -84.542     HC_GEN0011
...
```

## Commands

### find

Find soil data for coordinates.

**Options:**
- `--lat, -l` (required): Latitude in decimal degrees (-90 to 90)
- `--lon, -n` (required): Longitude in decimal degrees (-180 to 180)
- `--format, -f`: Output format - "summary" (default), "json", or "sol"
- `--output, -o`: Save to file instead of stdout

**Examples:**
```bash
# East Lansing, Michigan
soil-query find --lat=42.7 --lon=-84.5

# London, UK (JSON)
soil-query find --lat=51.5 --lon=-0.1 --format=json

# São Paulo, Brazil (save to file)
soil-query find --lat=-23.5 --lon=-46.6 --format=sol --output=sp.SOL
```

### definitions

List all soil property abbreviations with descriptions.
```bash
soil-query definitions
```

### explain

Explain a specific property abbreviation.
```bash
soil-query explain <ABBREVIATION>
```

**Examples:**
```bash
soil-query explain SLLL    # Wilting point
soil-query explain SDUL    # Field capacity
soil-query explain SBDM    # Bulk density
```

### stats

Show database statistics.
```bash
soil-query stats
```

## Global Options

- `--database, -d`: Path to SQLite database (default: `output/soil_data.db`)

**Example:**
```bash
soil-query --database=/path/to/custom.db find --lat=42.7 --lon=-84.5
```

## Database

The CLI requires the SQLite database created by `soil-query-parser`.

**Create database:**
```bash
# Parse .SOL files into database
cargo run --release --bin soil-query-parser -- \
    --input /path/to/sol_files \
    --output output/soil_data.db
```

**Database path:**

By default, the CLI looks for `output/soil_data.db`. You can specify a different path with `--database`.

## Data Coverage

- **Countries**: 225 worldwide
- **Profiles**: 1,984,797 soil profiles
- **Resolution**: ~10 km (5 arc-minute)
- **Depths**: 6 standard layers (5, 15, 30, 60, 100, 200 cm)
- **Properties**: 17 per layer (water retention, physical, chemical)

## Performance

- **Query time**: < 100ms for coordinate lookup
- **Accuracy**: < 5km to nearest profile (typically < 4km)
- **Database size**: 4.1 GB

## Examples

### Workflow: Get soil data for your farm
```bash
# Query your location
soil-query find --lat=42.7 --lon=-84.5

# Save as .SOL file for DSSAT
soil-query find --lat=42.7 --lon=-84.5 --format=sol --output=farm_soil.SOL

# Check what SLLL means
soil-query explain SLLL
```

### Workflow: Research project
```bash
# Get data as JSON for multiple locations
soil-query find --lat=40.7 --lon=-74.0 --format=json > nyc.json
soil-query find --lat=34.0 --lon=-118.2 --format=json > la.json

# Get database statistics
soil-query stats
```

## Troubleshooting

### Database not found
```
Error: Database not found: "output/soil_data.db"
```

**Solution**: Create the database first with `soil-query-parser`, or specify the correct path with `--database`.

### No profiles found
```
Error: No profiles found near coordinates
```

**Solution**: The location might be in a remote area. Try nearby coordinates or check if the database has data for that region.

### Invalid coordinates
```
Error: Invalid latitude: 95 (must be -90 to 90)
```

**Solution**: Check your coordinates. Latitude must be -90 to 90, longitude -180 to 180.

## License

See main README.md