# soil-query CLI

Command-line tool for querying global soil data with DSSAT-compatible outputs.

---

## Installation

### Pre-built Binaries (Recommended)

Download the latest binary for your platform from the [GitHub Releases](https://github.com/eusojk/soil-query/releases) page.

| Platform | File |
|----------|------|
| Linux (x86_64) | `soil-query-linux-x86_64` |
| macOS (Apple Silicon) | `soil-query-macos-aarch64` |
| macOS (Intel) | `soil-query-macos-x86_64` |
| Windows | `soil-query-windows-x86_64.exe` |

### From Source

```bash
cargo build --release --bin soil-query
```

Binary will be at `target/release/soil-query` (or `soil-query.exe` on Windows).

### Add to PATH (Optional)

```bash
# Linux/Mac
cp target/release/soil-query ~/.local/bin/

# Windows (PowerShell as Administrator)
Copy-Item target\release\soil-query.exe C:\Windows\System32\
```

---

## Usage

### Find Soil Data

```bash
# Summary output (default)
soil-query find --lat=42.7 --lon=-84.5

# JSON output
soil-query find --lat=42.7 --lon=-84.5 --format=json

# .SOL format (DSSAT-compatible)
soil-query find --lat=42.7 --lon=-84.5 --format=sol

# Save to file
soil-query find --lat=42.7 --lon=-84.5 --format=sol --output=soil.SOL
```

> **Windows PowerShell note**: Use `--lat=42.7` format (with `=`) to avoid issues with negative numbers being interpreted as flags.

### List Property Definitions

```bash
soil-query definitions
```

### Explain a Property

```bash
soil-query explain SLLL
soil-query explain SBDM
soil-query explain SLHW
```

### Database Statistics

```bash
soil-query stats
```

---

## Commands

### `find`

Find the nearest soil profile for any coordinates.

```bash
soil-query find --lat=<LAT> --lon=<LON> [OPTIONS]
```

**Options:**

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--lat` | `-l` | required | Latitude (-90 to 90) |
| `--lon` | `-n` | required | Longitude (-180 to 180) |
| `--format` | `-f` | `summary` | Output format: `summary`, `json`, or `sol` |
| `--output` | `-o` | stdout | Save to file instead of printing |

**Example output (summary):**
```
  Searching for soil data...
  Location: 42.700°, -84.500°
  Found profile!
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

---

### `definitions`

List all soil property abbreviations with descriptions and units.

```bash
soil-query definitions
```

---

### `explain`

Get a detailed explanation for a specific soil property abbreviation.

```bash
soil-query explain <ABBREVIATION>
```

**Examples:**
```bash
soil-query explain SLLL    # Lower limit (wilting point)
soil-query explain SDUL    # Drained upper limit (field capacity)
soil-query explain SBDM    # Bulk density
soil-query explain SLHW    # pH in water
soil-query explain SCEC    # Cation exchange capacity
```

---

### `stats`

Show database statistics and top countries by profile count.

```bash
soil-query stats
```

**Output:**
```
Database Statistics

  Total Profiles: 1984797

  Top 10 Countries:
    RU  421983 profiles (21.3%)
    CA  244237 profiles (12.3%)
    US  161724 profiles ( 8.1%)
    ...

  Database Size: 4102.95 MB
```

---

## Output Formats

### `summary` (default)

Human-readable table. Good for quick lookups and exploration.

### `json`

Machine-readable. Good for scripting and downstream processing.

```json
{
  "id": "US02450585",
  "location": {
    "lat": 42.708,
    "lon": -84.542,
    "country_code": "US"
  },
  "site": { ... },
  "layers": [ ... ]
}
```

### `sol`

DSSAT-compatible `.SOL` format, ready to drop into a crop model.

```
*US02450585     USA              Loam   200    ISRIC soilgrids + HC27
@SITE        COUNTRY          LAT     LONG SCS Family
 -99              US      42.708   -84.542     HC_GEN0011
...
```

---

## Global Options

| Flag | Short | Description |
|------|-------|-------------|
| `--database` | `-d` | Path to SQLite database (default: `output/soil_data.db`) |

```bash
soil-query --database=/path/to/custom.db find --lat=42.7 --lon=-84.5
```

---

## Database

The CLI requires the SQLite database built by `soil-query-parser`. See [`crates/soil-query-parser/README.md`](../soil-query-parser/README.md) for instructions on building it.

By default the CLI looks for `output/soil_data.db`. Override with `--database`.

---

## Example Workflows

### Get soil data for your farm
```bash
# Quick look
soil-query find --lat=42.7 --lon=-84.5

# Save as .SOL for DSSAT
soil-query find --lat=42.7 --lon=-84.5 --format=sol --output=farm_soil.SOL

# Understand what SLLL means
soil-query explain SLLL
```

### Research project — multiple locations
```bash
soil-query find --lat=40.7  --lon=-74.0  --format=json > nyc.json
soil-query find --lat=34.0  --lon=-118.2 --format=json > la.json
soil-query find --lat=-23.5 --lon=-46.6  --format=json > sao_paulo.json
```

---

## Troubleshooting

**Database not found:**
```
Error: Database not found: "output/soil_data.db"
```
Build or download the database first, or point to it with `--database`.

**No profiles found:**
```
Error: No profiles found near coordinates
```
The location may be in a remote or ocean area. Try nearby coordinates.

**Invalid coordinates:**
```
Error: Invalid latitude: 95 (must be -90 to 90)
```
Latitude must be -90 to 90, longitude -180 to 180.

---

## Current/Known Limitations

- **Database required**: The CLI does not query the remote API — it reads a local SQLite database.
- **No caching**: Each query hits the database directly.
- **No batch queries**: One location at a time (by design for simplicity in v0.1.0).