# Production Database Information

**Database**: `soil_data.db`  
**Created**: February 10, 2026  
**Version**: v0.1.0-phase2-production

---

## Statistics

- **Total Profiles**: 1,984,797
- **Countries Covered**: 225
- **Database Size**: 4,102.95 MB (4.1 GB)
- **Processing Time**: 9m 43s (583.90 seconds)
- **Processing Speed**: 3,399 profiles/second
- **Success Rate**: 100% (zero errors)

---

## Schema

### Main Table: `soil_profiles`
```sql
CREATE TABLE soil_profiles (
    rowid INTEGER PRIMARY KEY AUTOINCREMENT,
    id TEXT NOT NULL UNIQUE,
    country_code TEXT NOT NULL,
    country_code_alpha3 TEXT NOT NULL,
    lat REAL NOT NULL,
    lon REAL NOT NULL,
    scs_family TEXT NOT NULL,
    texture TEXT NOT NULL,
    max_depth_cm INTEGER NOT NULL,
    source TEXT NOT NULL,
    data TEXT NOT NULL,          -- Full profile as JSON
    created_at TEXT NOT NULL
);
```

**Indexes:**
- `idx_profile_id` on `id`
- `idx_country` on `country_code`

### Spatial Index: `soil_spatial_idx`
```sql
CREATE VIRTUAL TABLE soil_spatial_idx USING rtree(
    rowid,              -- Links to soil_profiles.rowid
    min_lat, max_lat,   -- Bounding box for point
    min_lon, max_lon    -- Bounding box for point
);
```

---

## Top 20 Countries by Profile Count

| Rank | Country | Code | Profiles | % of Total |
|------|---------|------|----------|------------|
| 1 | Russia | RU | 421,983 | 21.26% |
| 2 | Canada | CA | 244,237 | 12.31% |
| 3 | United States | US | 161,724 | 8.15% |
| 4 | China | CN | 120,107 | 6.05% |
| 5 | Brazil | BR | 102,731 | 5.18% |
| 6 | Australia | AU | 102,180 | 5.15% |
| 7 | Kazakhstan | KZ | 46,560 | 2.35% |
| 8 | India | IN | 40,467 | 2.04% |
| 9 | Argentina | AR | 39,912 | 2.01% |
| 10 | Algeria | DZ | 34,476 | 1.74% |

*(Remaining 215 countries account for 33.76% of profiles)*

---

## Query Examples

### Count profiles by country
```sql
SELECT country_code, COUNT(*) as count 
FROM soil_profiles 
GROUP BY country_code 
ORDER BY count DESC;
```

### Get profile by ID
```sql
SELECT * FROM soil_profiles WHERE id = 'US01006921';
```

### Find profiles in a region
```sql
SELECT id, lat, lon 
FROM soil_profiles 
WHERE lat BETWEEN 40 AND 45 
  AND lon BETWEEN -85 AND -80;
```

### Nearest neighbor query (simplified)
```sql
-- Find profiles near a coordinate (42.7, -84.5)
SELECT p.id, p.lat, p.lon,
       ABS(p.lat - 42.7) + ABS(p.lon - (-84.5)) as distance
FROM soil_profiles p
WHERE p.rowid IN (
    SELECT rowid FROM soil_spatial_idx
    WHERE min_lat >= 42.0 AND max_lat <= 43.5
      AND min_lon >= -85.0 AND max_lon <= -84.0
)
ORDER BY distance
LIMIT 1;
```

---

## Data Coverage

- **Resolution**: ~10 km (5 arc-minute)
- **Depths**: 6 layers (5, 15, 30, 60, 100, 200 cm)
- **Properties per layer**: 17 (water retention, physical, chemical)
- **Source**: ISRIC SoilGrids + HC27

---

## Usage Notes

### For Phase 3 (API Server):
- Load database into memory for fast queries: ~4.5 GB RAM
- Use R-tree spatial index for nearest neighbor searches
- Expected query time: < 10ms per coordinate lookup

### For Queries:
- All coordinates in WGS84 decimal degrees
- Latitude: -90 to 90
- Longitude: -180 to 180
- Missing data represented as NULL in JSON (was -99 in .SOL)

---

## File Integrity

**MD5 Checksum**: (run `md5sum soil_data.db` to generate)  
**SHA256 Checksum**: (run `sha256sum soil_data.db` to generate)

Verify database integrity:
```bash
sqlite3 soil_data.db "PRAGMA integrity_check;"
```

Expected output: `ok`

---

## Maintenance

### Rebuild Database
```bash
cargo run --release --bin soil-query-parser -- \
    --input /path/to/sol_files \
    --output ./output/soil_data.db \
    --report ./output/full_parse_report.json
```

### Vacuum (optimize size)
```bash
sqlite3 soil_data.db "VACUUM;"
```

### Backup
```bash
sqlite3 soil_data.db ".backup soil_data_backup.db"
```

---

