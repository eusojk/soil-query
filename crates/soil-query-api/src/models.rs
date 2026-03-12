//! API request and response models

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use soil_query::SoilProfile;

/// Query parameters for soil endpoint
#[derive(Debug, Deserialize, IntoParams)]
pub struct SoilQuery {
    /// Latitude in decimal degrees
    pub lat: f64,
    /// Longitude in decimal degrees
    pub lon: f64,
    /// Output format: "json" or "sol"
    #[serde(default = "default_format")]
    pub format: String,
}

fn default_format() -> String {
    "json".to_string()
}

/// Response for soil query (JSON format)
#[derive(Debug, Serialize, ToSchema)]
pub struct SoilResponse {
    pub profile: SoilProfile,
    pub distance_km: f64,
}

/// Health check response
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub profiles: usize,
}

/// Property definition
#[derive(Debug, Serialize, ToSchema)]
pub struct PropertyDefinition {
    pub abbreviation: String,
    pub full_name: String,
    pub unit: String,
    pub description: String,
}

/// Error response
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

