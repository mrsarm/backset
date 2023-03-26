use actix_web::http::StatusCode;
use actix_web::{error, HttpRequest, HttpResponse, ResponseError};
use anyhow::Error;
use serde::Serialize;
use sqlx::Error as SqlxError;

#[derive(Debug, Serialize)]
pub struct JsonError {
    pub error: String,
}

impl JsonError {
    pub fn new(detail: &str) -> Self {
        let text = match detail.rsplit_once("Json deserialize error: ") {
            None => detail,
            Some((_, t2)) => t2,
        };
        JsonError {
            error: String::from(text),
        }
    }
}

pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => {
            HttpResponse::UnsupportedMediaType().json(JsonError::new(&detail))
        }
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().json(JsonError::new(&detail))
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}

#[derive(thiserror::Error, Debug)]
pub enum BacksetError {
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
            BacksetError::Validation(_) => StatusCode::BAD_REQUEST,
            BacksetError::DB(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BacksetError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
