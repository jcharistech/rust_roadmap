
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use sqlx::Error as SqlxError;
use serde:: {Serialize,Deserialize};
use std::fmt;


// Error Handling
#[derive(Debug, Serialize, Deserialize)]
pub enum ApiError {
    InternalError(String),
    DatabaseError(String),
    NotFound(String),
    ValidationError(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            ApiError::InternalError(msg) => actix_web::HttpResponse::InternalServerError().json(msg),
            ApiError::DatabaseError(msg) => actix_web::HttpResponse::InternalServerError().json(msg),
            ApiError::NotFound(msg) => actix_web::HttpResponse::NotFound().json(msg),
            ApiError::ValidationError(msg) => actix_web::HttpResponse::BadRequest().json(msg),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::InternalError(msg) => write!(f, "Internal Server Error: {}", msg),
            ApiError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
        }
    }
}

impl From<SqlxError> for ApiError {
    fn from(e: SqlxError) -> Self {
        ApiError::DatabaseError(format!("Database error: {}", e))
    }
}
