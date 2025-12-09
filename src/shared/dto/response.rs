use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
}

/// Helper function for HTTP 200 OK JSON response.
pub fn http_ok(payload: impl Serialize) -> HttpResponse {
    HttpResponse::Ok().json(payload)
}

/// Helper function for HTTP 204 No Content response.
pub fn http_no_content() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

/// Helper function for HTTP 400 Bad Request JSON response.
pub fn http_bad_request(message: String) -> HttpResponse {
    HttpResponse::BadRequest().json(ErrorResponse { message })
}

/// Helper function for HTTP 500 Internal Server Error JSON response.
pub fn http_internal_server_error(message: String) -> HttpResponse {
    HttpResponse::InternalServerError().json(ErrorResponse { message })
}
