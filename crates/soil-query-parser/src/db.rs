//! Database operations for soil profiles

use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use soil_query::SoilProfile;
use std::path::Path;

/// Initialize the database with schema and indexes
pub fn init_database(db_path: &Path) -> Result<Connection> {
    // Delete existing database if it exists (full rebuild)
    if db_path.exists() {
        std::fs::remove_file(db_path).context("Failed to remove existing database")?;
    }

    // Create parent directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).context("Failed to create output directory")?;
    }

    let conn = Connection::open(db_path).context("Failed to create database")?;

    // Create schema
    create_schema(&conn)?;

    // Set pragmas for fast bulk insert
    set_bulk_insert_pragmas(&conn)?;

    Ok(conn)
}

/// Create database schema
fn create_schema(conn: &Connection) -> Result<()> {
    // Main table with INTEGER PRIMARY KEY for R-tree compatibility
    conn.execute(
        "CREATE TABLE soil_profiles (
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
            data TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    // Index on profile ID for fast lookups
    conn.execute("CREATE INDEX idx_profile_id ON soil_profiles(id)", [])?;

    // Index on country code for fast filtering
    conn.execute(
        "CREATE INDEX idx_country ON soil_profiles(country_code)",
        [],
    )?;

    // Spatial index (R-tree) - uses INTEGER rowid
    conn.execute(
        "CREATE VIRTUAL TABLE soil_spatial_idx USING rtree(
            rowid,
            min_lat, max_lat,
            min_lon, max_lon
        )",
        [],
    )?;

    Ok(())
}

/// Set pragmas for fast bulk insert
fn set_bulk_insert_pragmas(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "PRAGMA journal_mode = OFF;
         PRAGMA synchronous = OFF;
         PRAGMA cache_size = 10000;
         PRAGMA temp_store = MEMORY;",
    )?;
    Ok(())
}

/// Insert a soil profile into the database
pub fn insert_profile(conn: &Connection, profile: &SoilProfile) -> Result<()> {
    // Serialize the full profile to JSON
    let data_json = serde_json::to_string(profile).context("Failed to serialize profile")?;

    // Get current timestamp
    let timestamp = chrono::Utc::now().to_rfc3339();

    // Insert into main table
    conn.execute(
        "INSERT INTO soil_profiles (
            id, country_code, country_code_alpha3, lat, lon,
            scs_family, texture, max_depth_cm, source, data, created_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            profile.id,
            profile.location.country_code,
            profile.site.country_code_alpha3,
            profile.location.lat,
            profile.location.lon,
            profile.site.scs_family,
            profile.site.texture,
            profile.site.max_depth_cm,
            profile.metadata.source,
            data_json,
            timestamp,
        ],
    )?;

    // Get the rowid of the inserted row
    let rowid = conn.last_insert_rowid();

    // Insert into spatial index using the rowid
    conn.execute(
        "INSERT INTO soil_spatial_idx (rowid, min_lat, max_lat, min_lon, max_lon)
         VALUES (?1, ?2, ?2, ?3, ?3)",
        params![rowid, profile.location.lat, profile.location.lon,],
    )?;

    Ok(())
}

/// Finalize database after bulk insert (re-enable safety features)
pub fn finalize_database(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "PRAGMA journal_mode = DELETE;
         PRAGMA synchronous = FULL;",
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use soil_query::SoilProfile;

    #[test]
    fn test_init_database_creates_schema() -> Result<()> {
        let temp_db = std::env::temp_dir().join("test_init.db");

        let conn = init_database(&temp_db)?;

        // Verify main table exists
        {
            let mut stmt = conn.prepare(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='soil_profiles'",
            )?;
            let exists: bool = stmt.exists([])?;
            assert!(exists, "soil_profiles table should exist");
        } // stmt dropped here

        // Verify spatial index exists
        {
            let mut stmt = conn.prepare(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='soil_spatial_idx'",
            )?;
            let exists: bool = stmt.exists([])?;
            assert!(exists, "soil_spatial_idx table should exist");
        } // stmt dropped here

        // Cleanup
        drop(conn);
        std::fs::remove_file(&temp_db)?;

        Ok(())
    }

    #[test]
    fn test_insert_and_query_profile() -> Result<()> {
        let temp_db = std::env::temp_dir().join("test_insert.db");
        let conn = init_database(&temp_db)?;

        // Parse a test profile
        let content = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../test_data/GI.SOL"))?;
        let profiles = SoilProfile::from_sol_format(&content)?;
        let profile = &profiles[0];

        // Insert it
        insert_profile(&conn, profile)?;

        // Query it back
        {
            let mut stmt = conn.prepare("SELECT id, lat, lon FROM soil_profiles WHERE id = ?1")?;
            let mut rows = stmt.query(params![profile.id])?;

            if let Some(row) = rows.next()? {
                let id: String = row.get(0)?;
                let lat: f64 = row.get(1)?;
                let lon: f64 = row.get(2)?;

                assert_eq!(id, profile.id);
                assert!((lat - profile.location.lat).abs() < 0.001);
                assert!((lon - profile.location.lon).abs() < 0.001);
            } else {
                panic!("Profile not found in database");
            }
        } // stmt and rows dropped here

        // Cleanup
        drop(conn);
        std::fs::remove_file(&temp_db)?;

        Ok(())
    }
}
