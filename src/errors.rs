use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
pub enum ApiError {
    BadRequest(String),
    BlockingError(String),
    CacheError(String),
    CannotDecodeJwtToken(String),
    CannotEncodeJwtToken(String),
    InternalServerError(String),
    NotFound(String),
    ParseError(String),
    PoolError(String),
    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

/// Automatically convert ApiErrors to external Response Errors
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) => {
                // Explicitly convert `error` to `ErrorResponse`
                let error_response: ErrorResponse = error.into();
                HttpResponse::BadRequest().json(error_response)
            }
            ApiError::NotFound(message) => {
                // Explicitly convert `message` to `ErrorResponse`
                let error_response: ErrorResponse = message.into();
                HttpResponse::NotFound().json(error_response)
            }
            ApiError::ValidationError(errors) => {
                // Explicitly convert `errors.to_vec()` to `ErrorResponse`
                let error_response: ErrorResponse = errors.to_vec().into();
                HttpResponse::UnprocessableEntity().json(error_response)
            }
            ApiError::Unauthorized(error) => {
                // Explicitly convert `error` to `ErrorResponse`
                let error_response: ErrorResponse = error.into();
                HttpResponse::Unauthorized().json(error_response)
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

/// Utility to make transforming a string reference into an ErrorResponse
impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

/// Utility to make transforming a vector of strings into an ErrorResponse
impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}

/// Convert DBErrors to ApiErrors
impl From<DBError> for ApiError {
    fn from(error: DBError) -> ApiError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ApiError::BadRequest(message);
                }
                ApiError::InternalServerError("Unknown database error".into())
            }
            _ => ApiError::InternalServerError("Unknown database error".into()),
        }
    }
}
