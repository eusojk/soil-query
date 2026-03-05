//! API request handlers

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::{db::DbState, models::*};

/// Root endpoint
pub async fn root() -> &'static str {
    "soil-query API v0.1.0\n\nEndpoints:\n  GET /health\n  GET /soil?lat=<lat>&lon=<lon>&format=<json|sol>\n  GET /definitions"
}

/// Health check endpoint
pub async fn health(State(state): State<DbState>) -> Json<HealthResponse> {
    let status = if state.is_ready() { "ok" } else { "degraded - database not loaded" };
    Json(HealthResponse {
        status: status.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        profiles: state.profile_count,
    })
}

/// Get soil data for coordinates
pub async fn get_soil(
    State(state): State<DbState>,
    Query(params): Query<SoilQuery>,
) -> Result<Response, AppError> {
    // Check database is ready
    let connection = state.connection.ok_or_else(|| AppError::DatabaseError {
        message: "Database not loaded yet. Please try again shortly.".to_string(),
    })?;

    // Validate coordinates
    if !(-90.0..=90.0).contains(&params.lat) {
        return Err(AppError::InvalidCoordinate {
            message: format!("Invalid latitude: {} (must be -90 to 90)", params.lat),
        });
    }
    if !(-180.0..=180.0).contains(&params.lon) {
        return Err(AppError::InvalidCoordinate {
            message: format!("Invalid longitude: {} (must be -180 to 180)", params.lon),
        });
    }

    // Lock the database connection
    let conn = connection.lock()
        .map_err(|_| AppError::DatabaseError {
            message: "Failed to acquire database lock".to_string(),
        })?;

    // Find nearest profile
    let (profile, distance) = crate::db::find_nearest_profile(&conn, params.lat, params.lon)
        .map_err(|e| AppError::NotFound {
            message: format!("No soil data found near coordinates: {}", e),
        })?;

    // Return based on format
    match params.format.as_str() {
        "sol" => {
            let sol_content = profile.to_sol_format();
            Ok((
                StatusCode::OK,
                [("Content-Type", "text/plain")],
                sol_content,
            ).into_response())
        }
        _ => {
            let response = SoilResponse {
                profile,
                distance_km: distance,
            };
            Ok(Json(response).into_response())
        }
    }
}

/// Get property definitions
pub async fn get_definitions() -> Json<Vec<PropertyDefinition>> {
    Json(vec![
        PropertyDefinition {
            abbreviation: "SLLL".to_string(),
            full_name: "Lower limit (wilting point)".to_string(),
            unit: "cm³/cm³".to_string(),
            description: "Volumetric water content at wilting point".to_string(),
        },
        PropertyDefinition {
            abbreviation: "SDUL".to_string(),
            full_name: "Drained upper limit (field capacity)".to_string(),
            unit: "cm³/cm³".to_string(),
            description: "Volumetric water content at field capacity".to_string(),
        },
        PropertyDefinition {
            abbreviation: "SSAT".to_string(),
            full_name: "Upper limit, saturated".to_string(),
            unit: "cm³/cm³".to_string(),
            description: "Volumetric water content at saturation".to_string(),
        },
    ])
}

/// Custom error type for API
#[derive(Debug)]
pub enum AppError {
    InvalidCoordinate { message: String },
    NotFound { message: String },
    DatabaseError { message: String },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::InvalidCoordinate { message } => (StatusCode::BAD_REQUEST, message),
            AppError::NotFound { message } => (StatusCode::NOT_FOUND, message),
            AppError::DatabaseError { message } => (StatusCode::SERVICE_UNAVAILABLE, message),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}