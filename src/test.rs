//! Utils methods to write tests.

use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::test::read_body;
use actix_web::web::Bytes;

/// Check the response has the status passed, otherwise fail
/// with the response body printed out. If success
/// return the `Bytes` of the body, that can be later serialized
/// into a struct object with `serde_json::from_slice()``.
///
/// ```example
/// let body = assert_status(resp, StatusCode::OK).await;
/// let person: PersonPayload = serde_json::from_slice(&body).unwrap();
/// ```
pub async fn assert_status(resp: ServiceResponse, expected_status: StatusCode) -> Bytes {
    let status = resp.status();
    let body_bytes = read_body(resp).await;
    let body: &str = std::str::from_utf8(&body_bytes[..]).unwrap();
    assert_eq!(status, expected_status, "Response Body: {}", body);
    body_bytes
}
