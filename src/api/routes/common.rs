/*
* Error handling module for API responses.
*
* Provides standardized error responses for database errors and
* resource not found scenarios.
*/

use axum::{http::StatusCode, Json};
use utoipa::ToSchema;

/*
* Represents a structured error response.
*/
#[derive(serde::Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message describing what went wrong
    pub error: String,

    /// HTTP status code
    pub code: u16,
}

/*
* Generates a database error response.
*
* Formats the given error message and assigns an HTTP 500 status code.
*
* @param err The database error message
* @return Tuple containing the status code and error response JSON
*/
pub fn database_error(err: impl std::fmt::Display) -> (StatusCode, Json<ErrorResponse>) {
    let error_response = ErrorResponse {
        error: format!("Database error: {}", err),
        code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    };
    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
}

/*
* Generates a not found error response.
*
* Accepts a custom error message and assigns an HTTP 404 status code.
*
* @param message The not found error message
* @return Tuple containing the status code and error response JSON
*/
pub fn not_found_error(message: impl Into<String>) -> (StatusCode, Json<ErrorResponse>) {
    let error_response = ErrorResponse {
        error: message.into(),
        code: StatusCode::NOT_FOUND.as_u16(),
    };
    (StatusCode::NOT_FOUND, Json(error_response))
}

/*
* Provides a simple health check endpoint for monitoring.
*
* @return JSON response with status "ok" when the service is healthy
*/
pub async fn health_check() -> impl axum::response::IntoResponse {
    axum::Json(serde_json::json!({ "status": "ok" }))
}
