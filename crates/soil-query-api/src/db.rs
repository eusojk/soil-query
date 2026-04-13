//! Database operations

use anyhow::{Context, Result};
use rusqlite::Connection;
use soil_query::SoilProfile;
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Database state shared across handlers
#[derive(Clone)]
pub struct DbState {
    pub connection: Option<Arc<Mutex<Connection>>>,
    pub profile_count: usize,
}

impl DbState {
    pub fn is_ready(&self) -> bool {
        self.connection.is_some()
    }
}

/// Initialize database — returns Ok even if database is missing so the
/// service stays alive on Railway and can accept SSH connections.
pub async fn init_database(db_path: &Path) -> Result<DbState> {
    if !db_path.exists() {
        tracing::warn!(
            "Database file not found at {:?} — starting in degraded mode. \
             Upload soil_data.db to this path and restart.",
            db_path
        );
        return Ok(DbState {
            connection: None,
            profile_count: 0,
        });
    }

    // Open connection
    let conn = Connection::open(db_path).context("Failed to open database")?;

    // Get profile count
    let profile_count: usize = conn
        .query_row("SELECT COUNT(*) FROM soil_profiles", [], |row| row.get(0))
        .context("Failed to count profiles")?;

    tracing::info!("Database contains {} profiles", profile_count);

    Ok(DbState {
        connection: Some(Arc::new(Mutex::new(conn))),
        profile_count,
    })
}

/// Find the nearest soil profile to given coordinates
pub fn find_nearest_profile(conn: &Connection, lat: f64, lon: f64) -> Result<(SoilProfile, f64)> {
    // Use R-tree spatial index to find candidates
    // Then calculate actual distance and find the nearest

    // First, find profiles in a bounding box using R-tree
    let search_radius = 0.5; // degrees (~55km at equator)

    let mut stmt = conn.prepare(
        "SELECT p.data, p.lat, p.lon
         FROM soil_profiles p
         WHERE p.rowid IN (
             SELECT rowid FROM soil_spatial_idx
             WHERE min_lat >= ?1 AND max_lat <= ?2
               AND min_lon >= ?3 AND max_lon <= ?4
         )
         LIMIT 100",
    )?;

    let rows = stmt.query_map(
        [
            lat - search_radius,
            lat + search_radius,
            lon - search_radius,
            lon + search_radius,
        ],
        |row| {
            let data: String = row.get(0)?;
            let profile_lat: f64 = row.get(1)?;
            let profile_lon: f64 = row.get(2)?;
            Ok((data, profile_lat, profile_lon))
        },
    )?;

    // Find the closest profile
    let mut nearest: Option<(SoilProfile, f64)> = None;

    for row in rows {
        let (data, profile_lat, profile_lon) = row?;

        // Deserialize profile
        let mut profile: SoilProfile =
            serde_json::from_str(&data).context("Failed to deserialize profile")?;

        // Calculate distance
        let distance = haversine_distance(lat, lon, profile_lat, profile_lon);

        // Update metadata with distance
        profile.metadata.distance_km = Some(distance);

        // Check if this is closer
        match &nearest {
            None => nearest = Some((profile, distance)),
            Some((_, best_distance)) => {
                if distance < *best_distance {
                    nearest = Some((profile, distance));
                }
            }
        }
    }

    nearest.ok_or_else(|| anyhow::anyhow!("No profiles found near coordinates"))
}

/// Calculate distance between two points using Haversine formula
fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;

    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_KM * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haversine_distance() {
        // Distance between New York and Los Angeles (approx 3950 km)
        let distance = haversine_distance(40.7128, -74.0060, 34.0522, -118.2437);
        assert!((distance - 3950.0).abs() < 50.0); // Within 50km

        // Distance between same point should be 0
        let distance = haversine_distance(42.0, -84.0, 42.0, -84.0);
        assert!(distance < 0.1);
    }
}
