use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use actix_web_validator::Error;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;
use std::collections::HashMap;
use validator::{ValidationError, ValidationErrors};


///! Use to serialize a simple error with a static message.
#[derive(Debug, Serialize)]
pub struct InternalErrorPayload {
    pub error: &'static str,
}

///! Use to serialize a validation
///! with a string error and/or field validation errors.
///!
///! An error serialized as JSON looks like:
///
///! ```
///! {
///!   "error": "Validation error",
///!   "field_errors": {
///!     "name": [
///!       {
///!         "code": "length",
///!         "message": null,
///!         "params": { "min": 3, "value": "Sr" }
///!       }
///!     ]
///!   }
///! }
#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationErrorPayload {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_errors: Option<HashMap<String, Vec<ValidationError>>>,
}

impl ValidationErrorPayload {
    pub fn new(detail: String) -> Self {
        ValidationErrorPayload {
            error: detail,
            field_errors: None,
        }
    }
}

impl From<&ValidationErrors> for ValidationErrorPayload {
    fn from(error: &ValidationErrors) -> Self {
        let mut errors: HashMap<String, Vec<ValidationError>> = HashMap::new();
        errors.extend(error.field_errors().iter().map(|(k,v)| (String::from(*k), (*v).clone())));
        ValidationErrorPayload {
            error: "Validation error".to_owned(),
            field_errors: Some(errors),
        }
    }
}

pub fn json_error_handler(err: Error, _req: &HttpRequest) -> actix_web::error::Error {
    let json_error = match &err {
        Error::Validate(error) =>
            HttpResponse::BadRequest().json(ValidationErrorPayload::from(error)),
        Error::JsonPayloadError(error) =>
            HttpResponse::UnprocessableEntity()
                .json(ValidationErrorPayload::new(error.to_string())),
        _ =>
            HttpResponse::BadRequest()
                .json(ValidationErrorPayload::new(err.to_string())),
    };
    InternalError::from_response(err, json_error).into()
}


/// Main enum that implements the actix ResponseError
/// trait to be used as wrapper for different errors
/// in endpoint handlers.
#[derive(thiserror::Error, Debug)]
pub enum BacksetError {
    #[error("{0}")]
    StaticValidation(&'static str),

    #[error("{0}")]
    Validation(String),

    // TODO add more errors
    #[error(transparent)]
    DB(#[from] SqlxError),

    #[error(transparent)]
    Unexpected(#[from] Error),
}

impl ResponseError for BacksetError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::StaticValidation(_) | Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::DB(_) | Self::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        match self {
            Self::Validation(error) => {
                debug!("Validation error: {:?}", error);
                HttpResponse::build(status_code)
                    .json(ValidationErrorPayload::new(error.to_owned()))
            },
            Self::StaticValidation(error) => {
                debug!("Validation error: {:?}", error);
                HttpResponse::build(status_code)
                    .json(InternalErrorPayload { error })
            },
            _ => {
                error!("Unexpected error: {:?}", self);
                HttpResponse::build(status_code)
                    .json(InternalErrorPayload {
                        error: status_code.canonical_reason().unwrap_or("Unknown error")
                    })
            },
        }
    }
}
