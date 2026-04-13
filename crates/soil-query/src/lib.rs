//! # soil-query
//!
//! Core library for handling DSSAT-compatible soil data.
//!
//! This library provides data structures and parsers for working with
//! soil profile data in the .SOL format used by DSSAT crop models.
//!
//! ## Features
//!
//! - Parse .SOL files into structured data
//! - Serialize soil profiles back to .SOL format
//! - Handle missing data gracefully (Option types)
//! - Support for 225 countries at 10km resolution
//!
//! ## Example
//!
//! ```rust
//! use soil_query::SoilProfile;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let sol_content = r#"*GI02792815    GIB        Loam   200    ISRIC soilgrids + HC27
//! @SITE        COUNTRY          LAT     LONG SCS Family
//!  -99              GI       36.125   -5.375     HC_GEN0011
//! @ SCOM  SALB  SLU1  SLDR  SLRO  SLNF  SLPF  SMHB  SMPX  SMKE
//!     BK  0.10  6.00  0.50 75.00  1.00  1.00 SA001 SA001 SA001
//! @  SLB  SLMH  SLLL  SDUL  SSAT  SRGF  SSKS  SBDM  SLOC  SLCL  SLSI  SLCF  SLNI  SLHW  SLHB  SCEC  SADC
//!      5 A     0.143 0.258 0.409  1.00  1.52  1.20  4.31 23.86 30.14 -99.0  0.12  6.81 -99.0  23.6 -99.0
//!     15 A     0.155 0.270 0.413  0.85  1.39  1.22  3.65 25.79 29.29 -99.0  0.09  6.89 -99.0  20.9 -99.0
//!     30 AB    0.169 0.285 0.418  0.70  1.11  1.25  2.79 28.21 28.29 -99.0  0.07  6.96 -99.0  20.1 -99.0
//!     60 BA    0.182 0.299 0.424  0.50  0.95  1.30  1.79 30.57 27.14 -99.0  0.06  7.09 -99.0  21.0 -99.0
//!    100 B     0.181 0.296 0.422  0.38  0.98  1.36  1.02 30.36 26.57 -99.0  0.05  7.22 -99.0  21.1 -99.0
//!    200 BC    0.173 0.285 0.418  0.05  1.13  1.42  0.59 28.93 26.07 -99.0  0.05  7.41 -99.0  21.1 -99.0
//! "#;
//!
//! // Parse the .SOL file
//! let profiles = SoilProfile::from_sol_format(sol_content)?;
//!
//! // Access the first profile
//! let profile = &profiles[0];
//! println!("Profile ID: {}", profile.id);
//! println!("Location: {}, {}", profile.location.lat, profile.location.lon);
//! println!("Layers: {}", profile.layers.len());
//!
//! // Serialize back to .SOL format
//! let sol_output = profile.to_sol_format();
//! # Ok(())
//! # }
//! ```

pub mod definitions;
pub mod error;
pub mod parser;
pub mod types;

// Re-export main types for convenience
pub use error::{Result, SoilError};
pub use types::{Location, Metadata, SiteProperties, SiteWideProperties, SoilLayer, SoilProfile};
