//! Validation functions for soil profiles

use anyhow::{bail, Result};
use soil_query::SoilProfile;

/// Validate a soil profile before inserting into database
pub fn validate_profile(profile: &SoilProfile) -> Result<()> {
    validate_coordinates(profile.location.lat, profile.location.lon)?;
    validate_layer_count(profile)?;
    validate_layer_depths(profile)?;
    Ok(())
}

/// Validate latitude and longitude are within valid ranges
fn validate_coordinates(lat: f64, lon: f64) -> Result<()> {
    if !(-90.0..=90.0).contains(&lat) {
        bail!("Invalid latitude: {} (must be between -90 and 90)", lat);
    }
    if !(-180.0..=180.0).contains(&lon) {
        bail!("Invalid longitude: {} (must be between -180 and 180)", lon);
    }
    Ok(())
}

/// Validate profile has exactly 6 layers
fn validate_layer_count(profile: &SoilProfile) -> Result<()> {
    if profile.layers.len() != 6 {
        bail!(
            "Invalid layer count for profile {}: expected 6, got {}",
            profile.id,
            profile.layers.len()
        );
    }
    Ok(())
}

/// Validate layer depths are in expected order
fn validate_layer_depths(profile: &SoilProfile) -> Result<()> {
    let expected_depths = [5, 15, 30, 60, 100, 200];
    
    for (i, layer) in profile.layers.iter().enumerate() {
        if layer.slb != expected_depths[i] {
            bail!(
                "Invalid layer depth for profile {} at index {}: expected {}, got {}",
                profile.id,
                i,
                expected_depths[i],
                layer.slb
            );
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use soil_query::SoilProfile;

    #[test]
    fn test_validate_valid_profile() -> Result<()> {
        let content = std::fs::read_to_string("../../test_data/GI.SOL")?;
        let profiles = SoilProfile::from_sol_format(&content)?;
        let profile = &profiles[0];
        
        // Should not error
        validate_profile(profile)?;
        
        Ok(())
    }

    #[test]
    fn test_validate_coordinates_valid() {
        assert!(validate_coordinates(36.125, -5.375).is_ok());
        assert!(validate_coordinates(0.0, 0.0).is_ok());
        assert!(validate_coordinates(90.0, 180.0).is_ok());
        assert!(validate_coordinates(-90.0, -180.0).is_ok());
    }

    #[test]
    fn test_validate_coordinates_invalid_lat() {
        assert!(validate_coordinates(91.0, 0.0).is_err());
        assert!(validate_coordinates(-91.0, 0.0).is_err());
        assert!(validate_coordinates(100.0, 0.0).is_err());
    }

    #[test]
    fn test_validate_coordinates_invalid_lon() {
        assert!(validate_coordinates(0.0, 181.0).is_err());
        assert!(validate_coordinates(0.0, -181.0).is_err());
        assert!(validate_coordinates(0.0, 200.0).is_err());
    }

    #[test]
    fn test_validate_layer_depths() -> Result<()> {
        let content = std::fs::read_to_string("../../test_data/GI.SOL")?;
        let profiles = SoilProfile::from_sol_format(&content)?;
        let profile = &profiles[0];
        
        // Should pass - GI.SOL has correct depths
        validate_layer_depths(profile)?;
        
        Ok(())
    }
}

