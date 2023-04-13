use crate::errors::AppError;
use actix_web::HttpResponse;
use sqlx::{Postgres, Transaction};

pub type Result<T> = core::result::Result<T, AppError>;

pub type Tx<'a> = Transaction<'a, Postgres>;

pub type HttpResult = Result<HttpResponse>;
