//! Core data types for soil profiles

use serde::{Deserialize, Serialize};

/// Geographic location of a soil profile
///
/// Contains WGS84 coordinates and ISO country code.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// Latitude in decimal degrees (-90 to 90)
    pub lat: f64,
    /// Longitude in decimal degrees (-180 to 180)
    pub lon: f64,
    /// ISO 3166-1 alpha-2 country code (e.g., "US", "GI", "BM")
    pub country_code: String,
}

/// Site-level properties from @SITE section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SiteProperties {
    /// ISO 3166-1 alpha-3 country code (3 letters, from header line)
    pub country_code_alpha3: String,
    /// SCS soil family classification
    pub scs_family: String,
    /// Soil texture classification
    pub texture: String,
    /// Maximum depth in centimeters
    pub max_depth_cm: u32,
}

/// Site-wide properties from @ SCOM section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SiteWideProperties {
    /// Color, moist, Munsell hue
    pub scom: String,
    /// Albedo, fraction
    pub salb: f64,
    /// Evaporation limit, mm
    pub slu1: f64,
    /// Drainage rate, fraction day-1
    pub sldr: f64,
    /// Runoff curve number (Soil Conservation Service)
    pub slro: f64,
    /// Mineralization factor, 0 to 1 scale
    pub slnf: f64,
    /// Photosynthesis factor, 0 to 1 scale
    pub slpf: f64,
    /// pH in buffer determination method, code
    pub smhb: String,
    /// Phosphorus determination code
    pub smpx: String,
    /// Potassium determination method, code
    pub smke: String,
}

/// Individual soil layer data from @  SLB section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoilLayer {
    /// Depth to bottom of layer, cm
    pub slb: u32,
    /// Master horizon
    pub slmh: String,
    /// Lower limit (wilting point), cm³/cm³
    /// None represents missing data (was -99 in .SOL file)
    pub slll: Option<f64>,
    /// Drained upper limit (field capacity), cm³/cm³
    pub sdul: Option<f64>,
    /// Upper limit, saturated, cm³/cm³
    pub ssat: Option<f64>,
    /// Root growth factor, 0.0 to 1.0
    pub srgf: Option<f64>,
    /// Saturated hydraulic conductivity, cm/h
    pub ssks: Option<f64>,
    /// Bulk density, g/cm³
    pub sbdm: Option<f64>,
    /// Organic carbon, %
    pub sloc: Option<f64>,
    /// Clay (<0.002 mm), %
    pub slcl: Option<f64>,
    /// Silt (0.05 to 0.002 mm), %
    pub slsi: Option<f64>,
    /// Coarse fraction (>2 mm), %
    pub slcf: Option<f64>,
    /// Total nitrogen, %
    pub slni: Option<f64>,
    /// pH in water
    pub slhw: Option<f64>,
    /// pH in buffer
    pub slhb: Option<f64>,
    /// Cation exchange capacity, cmol/kg
    pub scec: Option<f64>,
    /// Anion adsorption capacity
    pub sadc: Option<f64>,
}

/// Metadata about the soil profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    /// Data source description
    pub source: String,
    /// Distance to query point in kilometers (for API responses)
    pub distance_km: Option<f64>,
}

/// Complete soil profile with all properties and layers
///
/// A soil profile contains site information, location data,
/// and typically 6 soil layers at standard depths:
/// 5, 15, 30, 60, 100, and 200 cm.
///
/// # Example
///
/// ```rust
/// use soil_query::SoilProfile;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let content = std::fs::read_to_string("tests/data/GI.SOL")?;
/// let profiles = SoilProfile::from_sol_format(&content)?;
///
/// for profile in profiles {
///     println!("{}: {} layers", profile.id, profile.layers.len());
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoilProfile {
    /// Unique identifier for this profile
    pub id: String,
    /// Geographic location
    pub location: Location,
    /// Site properties
    pub site: SiteProperties,
    /// Site-wide properties
    pub properties: SiteWideProperties,
    /// Soil layers (typically 6 layers)
    pub layers: Vec<SoilLayer>,
    /// Metadata
    pub metadata: Metadata,
}