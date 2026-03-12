# soil-query

> Global soil data with DSSAT-compatible outputs

Query soil profiles for 225 countries at 10km resolution. Built for the DSSAT crop modeling community.

[**Interactive Map Explorer**](https://soilmap.josuekpodo.com) · [**API**](https://soil-query-production.up.railway.app)

---

## About

`soil-query` currently makes DSSAT-compatible soil data accessible without downloading a 4.5 GB dataset. It wraps 1,984,797 soil profiles from 225 countries behind a simple API, CLI, and interactive web map.


---

## Quick Start

### Web Interface

Click anywhere on the **[explorer map](https://soilmap.josuekpodo.com)** to get soil data for that location.

### API

```bash
# JSON output
curl "https://soil-query-production.up.railway.app/soil?lat=42.7&lon=-84.5&format=json"

# .SOL format output (DSSAT-compatible)
curl "https://soil-query-production.up.railway.app/soil?lat=42.7&lon=-84.5&format=sol"

# Health check
curl https://soil-query-production.up.railway.app/health
```

### CLI

```bash
# Build from source
cargo build --release --bin soil-query

# Find soil data for any location
soil-query find --lat=42.7 --lon=-84.5

# Get JSON output
soil-query find --lat=42.7 --lon=-84.5 --format=json

# Save as .SOL file
soil-query find --lat=42.7 --lon=-84.5 --format=sol --output=soil.SOL
```

### As a Library

TODO

---

## Project Structure

```
soil-query/
├── crates/
│   ├── soil-query/              # Core library: data structures, parser, serializer
│   ├── soil-query-parser/       # One-time tool to build the SQLite database from .SOL files
│   ├── soil-query-api/          # REST API server (Axum)
│   └── soil-query-cli/          # CLI tool
├── web/                         # Frontend (MapLibre, vanilla JS)
├── test_data/                   # 10 sample .SOL files for testing
└── output/                      # Generated database and reports (gitignored)
```

Each component has its own README:

- [`crates/soil-query-parser/README.md`](crates/soil-query-parser/README.md): building the database from raw .SOL files
- [`crates/soil-query-cli/README.md`](crates/soil-query-cli/README.md): CLI commands, installation, examples
- [`crates/soil-query-api/README.md`](crates/soil-query-api/README.md): API endpoints, deployment, configuration
- [`web/README.md`](web/README.md): frontend setup, deployment, file structure

---

## Development

```bash
# Run all tests
cargo test --all

# Lint
cargo clippy --all-targets --all-features

# Build all binaries
cargo build --release

# Run the API locally
cargo run --release --bin soil-query-api

# View generated docs
cargo doc --open
```

---

## Data Source


Han, Eunjin; Ines, Amor; Koo, Jawoo, 2015. "*Global High-Resolution Soil Profile Database for Crop Modeling Applications.*", [http://dx.doi.org/10.7910/DVN/1PEEY0](http://dx.doi.org/10.7910/DVN/1PEEY0), Harvard Dataverse, V1. 


---

## License

TODO

## Contributing

TODO