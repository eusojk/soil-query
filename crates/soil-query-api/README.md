# soil-query-api

REST API server for querying global soil data.

## Features

-  Query soil data by coordinates
-  Spatial indexing with R-tree for fast lookups
-  JSON and .SOL output formats
-  1.98 million soil profiles from 225 countries
-  CORS enabled for web applications
-  Health check endpoint

## Quick Start
```bash
# Run the API server
cargo run --release --bin soil-query-api

# Server starts on http://127.0.0.1:3000
```

## API Endpoints

### GET /health

Health check endpoint.

**Response:**
```json
{
  "status": "ok",
  "version": "0.1.0",
  "profiles": 1984797
}
```

### GET /soil

Query soil data for coordinates.

**Parameters:**
- `lat` (required): Latitude in decimal degrees (-90 to 90)
- `lon` (required): Longitude in decimal degrees (-180 to 180)
- `format` (optional): Output format - "json" (default) or "sol"

**Example (JSON):**
```bash
curl "http://127.0.0.1:3000/soil?lat=42.7&lon=-84.5&format=json"
```

**Example (.SOL):**
```bash
curl "http://127.0.0.1:3000/soil?lat=42.7&lon=-84.5&format=sol"
```

**JSON Response:**
```json
{
  "profile": {
    "id": "US02450585",
    "location": {
      "lat": 42.708,
      "lon": -84.542,
      "country_code": "US"
    },
    "site": {
      "texture": "Loam",
      "max_depth_cm": 200
    },
    "layers": [...]
  },
  "distance_km": 3.545
}
```

### GET /definitions

Get property definitions for soil data abbreviations.

**Response:**
```json
[
  {
    "abbreviation": "SLLL",
    "full_name": "Lower limit (wilting point)",
    "unit": "cm³/cm³",
    "description": "Volumetric water content at wilting point"
  }
]
```

## Configuration

### Environment Variables

- `DATABASE_PATH`: Path to SQLite database (default: `output/soil_data.db`)

### Custom Database Path
```bash
DATABASE_PATH=/path/to/custom.db cargo run --release --bin soil-query-api
```

## Performance

- **Database size**: 4.1 GB (1,984,797 profiles)
- **Query time**: < 50ms average
- **Spatial search**: R-tree indexed for fast lookups
- **Nearest neighbor**: Haversine distance calculation

## Error Handling

The API returns appropriate HTTP status codes:

- `200 OK`: Success
- `400 Bad Request`: Invalid parameters (e.g., lat > 90)
- `404 Not Found`: No soil data found near coordinates
- `500 Internal Server Error`: Database error

**Example error:**
```json
{
  "error": "Invalid latitude: 95 (must be -90 to 90)"
}
```

## Development
```bash
# Run tests
cargo test --package soil-query-api

# Run with debug logging
RUST_LOG=debug cargo run --bin soil-query-api
```

## Architecture

- **Framework**: Axum (async Rust web framework)
- **Database**: SQLite with R-tree spatial index
- **Concurrency**: Arc<Mutex<Connection>> for thread-safe access
- **Distance**: Haversine formula for accurate Earth distances

## License

See main README.md in root directory.
