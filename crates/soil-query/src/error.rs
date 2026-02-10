//! Error types for soil-query operations

use std::fmt;

/// Result type alias for soil-query operations
pub type Result<T> = std::result::Result<T, SoilError>;

/// Errors that can occur when working with soil data
#[derive(Debug, Clone, PartialEq)]
pub enum SoilError {
    /// Failed to parse .SOL format
    ParseError(String),
    
    /// Invalid coordinate values
    InvalidCoordinate { lat: f64, lon: f64 },
    
    /// Missing required field in .SOL file
    MissingField(String),
    
    /// Invalid numeric value
    InvalidValue { field: String, value: String },
}

impl fmt::Display for SoilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SoilError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            SoilError::InvalidCoordinate { lat, lon } => {
                write!(f, "Invalid coordinates: lat={}, lon={}", lat, lon)
            }
            SoilError::MissingField(field) => write!(f, "Missing required field: {}", field),
            SoilError::InvalidValue { field, value } => {
                write!(f, "Invalid value for {}: {}", field, value)
            }
        }
    }
}

impl std::error::Error for SoilError {}