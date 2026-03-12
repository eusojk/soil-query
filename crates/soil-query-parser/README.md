# soil-query-parser

One-time tool to parse raw `.SOL` files into the SQLite database used by `soil-query-api` and `soil-query-cli`.

This is only needed if building the database from scratch. The production database (`soil_data.db`) is already deployed on Railway.

---

## What it does

1. Reads all `.SOL` files from an input directory (one file per country, 225 total)
2. Parses each soil profile using the `soil-query` core library
3. Inserts profiles into a SQLite database with an R-tree spatial index
4. Optionally validates profiles and generates a statistics report

---

## Usage

```bash
cargo run --release --bin soil-query-parser -- \
    --input /path/to/sol_files \
    --output ./output/soil_data.db \
    --report ./output/report.json \
    --verbose
```

### Options

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--input` | `-i` | required | Directory containing `.SOL` files |
| `--output` | `-o` | required | Output path for the SQLite database |
| `--report` | `-r` | none | Optional path to write a JSON statistics report |
| `--verbose` | `-v` | false | Print progress per file |

---

## Example Output

```
soil-query-parser v0.1.0
Found 225 .SOL files

Processing: US.SOL
  Parsed 161724 profiles from US.SOL
  Progress: 161724/161724 (100%)
  US.SOL: 161724 profiles

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

---

## Test Run (Small Dataset)

Before parsing all 225 countries, test with the included sample files:

```bash
cargo run --release --bin soil-query-parser -- \
    --input ./test_data \
    --output ./output/test.db \
    --verbose
```

`test_data/` contains 10 representative `.SOL` files (~161,746 profiles, completes in ~33 seconds).

---

## Database Schema

```sql
-- Main profile table
CREATE TABLE soil_profiles (
    rowid   INTEGER PRIMARY KEY AUTOINCREMENT,
    id      TEXT    NOT NULL UNIQUE,
    country_code TEXT NOT NULL,
    lat     REAL    NOT NULL,
    lon     REAL    NOT NULL,
    -- additional columns for site properties and serialized layer data
    ...
);

-- Spatial index for fast coordinate queries
CREATE VIRTUAL TABLE soil_spatial_idx USING rtree(
    rowid,
    min_lat, max_lat,
    min_lon, max_lon
);
```

The R-tree spatial index is what enables sub-50ms nearest-neighbor lookups at query time.

---

## Performance

| Dataset | Files | Profiles | Duration | Speed |
|---------|-------|----------|----------|-------|
| Test (sample) | 10 | 161,746 | 32.75s | 4,939/s |
| Production (full) | 225 | 1,984,797 | 583.90s | 3,399/s |

Parsing is CPU-bound and single-threaded. The full dataset takes roughly 10 minutes on a modern laptop.

---

## Output

The parser produces a single SQLite file (`soil_data.db`, ~4.1 GB). This file is:

- Used directly by `soil-query-cli` (via `--database` flag or default path)
- Deployed to Railway's persistent volume at `/data/soil_data.db` for `soil-query-api`

---

## Data Source

Input `.SOL` files come from:

> International Research Institute for Climate and Society (IRI); Michigan State University (MSU); HarvestChoice, International Food Policy Research Institute (IFPRI), 2015, "SoilGrids-for-DSSAT-10km v1.0 (by country).zip", Global High-Resolution Soil Profile Database for Crop Modeling Applications,[https://doi.org/10.7910/DVN/1PEEY0/OG0STZ]( https://doi.org/10.7910/DVN/1PEEY0/OG0STZ), Harvard Dataverse, V2.
