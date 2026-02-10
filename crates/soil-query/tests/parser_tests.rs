//! Integration tests for .SOL file parsing

use soil_query::{SoilProfile, Result};

#[test]
fn test_parse_single_profile_gi() -> Result<()> {
    // Arrange: Read the GI.SOL file
    let content = std::fs::read_to_string("tests/data/GI.SOL")
        .expect("Failed to read GI.SOL test file");
    
    // Act: Parse the file
    let profiles = SoilProfile::from_sol_format(&content)?;
    
    // Assert: Should have exactly 1 profile
    assert_eq!(profiles.len(), 1, "GI.SOL should contain 1 profile");
    
    let profile = &profiles[0];
    
    // Assert: Check profile ID
    assert_eq!(profile.id, "GI02792815");
    
    // Assert: Check location
    assert_eq!(profile.location.country_code, "GI");
    assert!((profile.location.lat - 36.125).abs() < 0.001);
    assert!((profile.location.lon - (-5.375)).abs() < 0.001);
    
    // Assert: Check site properties
    assert_eq!(profile.site.texture, "Loam");
    assert_eq!(profile.site.max_depth_cm, 200);
    assert_eq!(profile.site.scs_family, "HC_GEN0011");
    
    // Assert: Check we have 6 layers
    assert_eq!(profile.layers.len(), 6);
    
    // Assert: Check first layer depth
    assert_eq!(profile.layers[0].slb, 5);
    
    // Assert: Check last layer depth
    assert_eq!(profile.layers[5].slb, 200);
    
    Ok(())
}

#[test]
fn test_parse_multiple_profiles_bm() -> Result<()> {
    // Arrange: Read the BM.SOL file
    let content = std::fs::read_to_string("tests/data/BM.SOL")
        .expect("Failed to read BM.SOL test file");
    
    // Act: Parse the file
    let profiles = SoilProfile::from_sol_format(&content)?;
    
    // Assert: Should have exactly 5 profiles (based on BM_SOL.txt content)
    assert_eq!(profiles.len(), 5, "BM.SOL should contain 5 profiles");
    
    // Assert: Check first profile ID
    assert_eq!(profiles[0].id, "BM02986503");
    
    // Assert: Check last profile ID
    assert_eq!(profiles[4].id, "BM02990823");
    
    // Assert: All profiles should be from Bermuda
    for profile in &profiles {
        assert_eq!(profile.location.country_code, "BM");
    }
    
    Ok(())
}

#[test]
fn test_missing_values_represented_as_none() -> Result<()> {
    // Arrange: Read GI.SOL which has -99 values
    let content = std::fs::read_to_string("tests/data/GI.SOL")
        .expect("Failed to read GI.SOL test file");
    
    // Act
    let profiles = SoilProfile::from_sol_format(&content)?;
    let first_layer = &profiles[0].layers[0];
    
    // Assert: SLCF, SLHB, SADC should be None (were -99 in file)
    assert_eq!(first_layer.slcf, None, "SLCF should be None for missing data");
    assert_eq!(first_layer.slhb, None, "SLHB should be None for missing data");
    assert_eq!(first_layer.sadc, None, "SADC should be None for missing data");
    
    // Assert: SLLL should have a value (not -99)
    assert!(first_layer.slll.is_some(), "SLLL should have a value");
    assert!((first_layer.slll.unwrap() - 0.143).abs() < 0.001);
    
    Ok(())
}

#[test]
fn test_country_code_alpha3_parsed() -> Result<()> {
    // Arrange
    let content = std::fs::read_to_string("tests/data/GI.SOL")
        .expect("Failed to read GI.SOL test file");
    
    // Act
    let profiles = SoilProfile::from_sol_format(&content)?;
    
    // Assert: Check alpha-3 country code from header
    assert_eq!(profiles[0].site.country_code_alpha3, "GIB");
    
    // Assert: Check alpha-2 country code from @SITE line
    assert_eq!(profiles[0].location.country_code, "GI");
    
    Ok(())
}

#[test]
fn test_roundtrip_parse_and_serialize() -> Result<()> {
    // Arrange: Read original file
    let original_content = std::fs::read_to_string("tests/data/GI.SOL")
        .expect("Failed to read GI.SOL test file");
    
    // Act: Parse then serialize
    let profiles = SoilProfile::from_sol_format(&original_content)?;
    let serialized = profiles[0].to_sol_format();
    
    // Parse the serialized version again
    let reparsed = SoilProfile::from_sol_format(&serialized)?;
    
    // Assert: Should have same data
    assert_eq!(profiles[0].id, reparsed[0].id);
    assert_eq!(profiles[0].location, reparsed[0].location);
    assert_eq!(profiles[0].site, reparsed[0].site);
    assert_eq!(profiles[0].layers.len(), reparsed[0].layers.len());
    
    // Check first layer values are preserved
    let original_layer = &profiles[0].layers[0];
    let reparsed_layer = &reparsed[0].layers[0];
    assert_eq!(original_layer.slb, reparsed_layer.slb);
    assert_eq!(original_layer.slll, reparsed_layer.slll);
    assert_eq!(original_layer.slcf, reparsed_layer.slcf); // Should be None
    
    Ok(())
}


