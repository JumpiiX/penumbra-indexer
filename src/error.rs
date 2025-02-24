use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum ApiError {
    DatabaseError(SqlxError),
    NotFound(String),
    BadRequest(String),
    InternalError(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::DatabaseError(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalError(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
        };

        let body = Json(ErrorResponse {
            status: status.to_string(),
            message,
        });

        (status, body).into_response()
    }
}

impl From<SqlxError> for ApiError {
    fn from(error: SqlxError) -> Self {
        match error {
            SqlxError::RowNotFound => ApiError::NotFound("Resource not found".to_string()),
            _ => ApiError::DatabaseError(error),
        }
    }
}