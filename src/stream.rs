//! Utils to deal with streams data types.

use crate::core::Result;
use crate::errors::AppError;
use actix_http::error::PayloadError;
use actix_web::web::Bytes;
use awc::ResponseBody;
use futures_core::stream::Stream;

/// Read body as string.
/// The content has to be encoded in UTF-8, otherwise
/// AppError::Unexpected is returned.
pub async fn read_body<S>(body: ResponseBody<S>) -> Result<String>
where
    S: Stream<Item = core::result::Result<Bytes, PayloadError>>,
{
    let bytes = body.await.unwrap().to_vec();
    String::from_utf8(bytes).map_err(|e| AppError::Unexpected(e.into()))
}
