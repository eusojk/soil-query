# soil-query

> Global soil data with DSSAT-compatible outputs

Query soil profiles for 225 countries at 10km resolution. Built for the DSSAT crop modeling community.

[**Interactive Map Explorer**](https://soilmap.josuekpodo.com) · [**Interactive API Docs (Swagger UI)**](https://soil-query-production.up.railway.app/swagger-ui/)

---

## About

`soil-query` currently makes DSSAT-compatible soil data accessible without downloading a 4.5 GB dataset. It wraps 1,984,797 soil profiles from 225 countries behind a simple API, CLI, and interactive web map. To get started, check out the explorer or API docs linked above.


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

For more details, check out the appropriate README file:

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

## Data Source Citation


Han, Eunjin; Ines, Amor; Koo, Jawoo, 2015. "*Global High-Resolution Soil Profile Database for Crop Modeling Applications.*", [http://dx.doi.org/10.7910/DVN/1PEEY0](http://dx.doi.org/10.7910/DVN/1PEEY0), Harvard Dataverse, V1. 


---

## License

Licensed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.


## Contributing

TODO