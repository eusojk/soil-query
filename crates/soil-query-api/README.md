# soil-query-api

REST API server for querying global soil data.

## Features

- Query soil data by coordinates (lat/lon)
- Spatial indexing with R-tree for fast lookups
- JSON and .SOL output formats
- 1,984,797 soil profiles from 225 countries
- CORS enabled for web applications
- Health check endpoint

---

## Quick Start

### Local Development

```bash
# Run the API server
cargo run --release --bin soil-query-api

# Server starts on http://127.0.0.1:3000
```

### Production (Railway)

The API is currently live at:

```
https://soil-query-production.up.railway.app
```

```bash
curl "https://soil-query-production.up.railway.app/soil?lat=42.7&lon=-84.5&format=json"
```

---

## API Endpoints

### GET /health

Health check returns server status and total profile count.

```bash
curl https://soil-query-production.up.railway.app/health
```

**Response:**
```json
{
  "status": "ok",
  "version": "0.1.0",
  "profiles": 1984797
}
```

---

### GET /soil

Query soil data for any coordinates.

**Parameters:**

| Name | Required | Description |
|------|----------|-------------|
| `lat` | ✅ | Latitude in decimal degrees (-90 to 90) |
| `lon` | ✅ | Longitude in decimal degrees (-180 to 180) |
| `format` | ❌ | `json` (default) or `sol` |


**Example .SOL:**
```bash
curl "https://soil-query-production.up.railway.app/soil?lat=42.7&lon=-84.5&format=sol"
```
.SOL Response:
```
*US02450585     USA              Loam   200    ISRIC soilgrids + HC27
@SITE        COUNTRY          LAT     LONG SCS Family
 -99              US      42.708   -84.542     HC_GEN0011
@ SCOM  SALB  SLU1  SLDR  SLRO  SLNF  SLPF  SMHB  SMPX  SMKE
    BK  0.10  6.00  0.50 75.00  1.00  1.00 SA001 SA001 SA001
@  SLB  SLMH  SLLL  SDUL  SSAT  SRGF  SSKS  SBDM  SLOC  SLCL  SLSI  SLCF  SLNI  SLHW  SLHB  SCEC  SADC
     5 A     0.101 0.227 0.389  1.00  1.24  1.56  2.17 16.61 39.11 -99.0  0.12  6.25 -99.0 20.70 -99.0
    15 A     0.112 0.237 0.391  0.85  1.05  1.58  1.83 18.40 38.29 -99.0  0.09  6.32 -99.0 18.10 -99.0
    30 AB    0.125 0.251 0.395  0.70  0.84  1.61  1.40 20.65 37.17 -99.0  0.07  6.41 -99.0 17.60 -99.0
    60 BA    0.138 0.265 0.399  0.50  0.68  1.66  0.89 22.89 35.92 -99.0  0.06  6.52 -99.0 18.40 -99.0
   100 B     0.138 0.263 0.399  0.38  0.70  1.72  0.52 22.82 35.31 -99.0  0.05  6.66 -99.0 18.50 -99.0
   200 BC    0.129 0.251 0.395  0.05  0.83  1.78  0.29 21.25 35.03 -99.0  0.05  6.85 -99.0 18.40 -99.0

```

**Example JSON:**
```bash
curl "https://soil-query-production.up.railway.app/soil?lat=42.7&lon=-84.5&format=json"
```

JSON Response:
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

---

### GET /definitions

Returns definitions for all soil property abbreviations.

```bash
curl https://soil-query-production.up.railway.app/definitions
```

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

---

## Error Handling

| Status | Meaning |
|--------|---------|
| `200 OK` | Success |
| `400 Bad Request` | Invalid parameters (e.g. lat > 90) |
| `404 Not Found` | No soil data found near coordinates |
| `500 Internal Server Error` | Database error |

**Example error response:**
```json
{
  "error": "Invalid latitude: 95 (must be -90 to 90)"
}
```

---

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_PATH` | `output/soil_data.db` | Path to SQLite database |
| `PORT` | `3000` | Port to bind to (set automatically by Railway) |

### Custom Database Path

```bash
DATABASE_PATH=/path/to/custom.db cargo run --release --bin soil-query-api
```

---

## Deployment

The API binds to `0.0.0.0` and reads `$PORT` from the environment, making it compatible with any hosting platform that supports containerized or native Rust binaries.

**Requirements for any deployment:**
- `DATABASE_PATH` env var pointing to `soil_data.db` (4.1 GB)
- Persistent storage for the database file
- `PORT` env var set by the host (or hardcoded for self-hosted)

---

### Railway (current)

Deployed with a persistent volume mounted at `/data`. Database lives at `/data/soil_data.db`.

Key notes:
- Graceful degraded startup if the database is missing (keeps the container alive for SSH/maintenance)
- Docker image includes `curl` for in-container file transfers
- See `railway.toml` and `Dockerfile` in the root for configuration

---

### IIS (Windows Server)

> [Coming soon]

Planned deployment on IIS using the [HttpPlatformHandler](https://learn.microsoft.com/en-us/iis/extensions/httpplatformhandler/httpplatformhandler-configuration-reference) module, which forwards requests from IIS to the Rust binary. Notes will be added here once tested.

---

### Other Platforms

The API should work on any platform that can run a Rust binary or Docker container (Fly.io, Render, VPS, etc.). Set `DATABASE_PATH` and `PORT` and point a reverse proxy at the process.

---

## Performance

- **Database size**: 4.1 GB (1,984,797 profiles)
- **Query time**: < 50ms average
- **Spatial search**: R-tree indexed
- **Distance**: Haversine formula

---

## Architecture

- **Framework**: Axum (async Rust)
- **Database**: SQLite with R-tree spatial index
- **Concurrency**: `Arc<Mutex<Connection>>` for thread-safe DB access
- **Hosting**: Railway (europe-west4)

---

## Development

```bash
# Run tests
cargo test --package soil-query-api

# Run with debug logging
RUST_LOG=debug cargo run --bin soil-query-api
```