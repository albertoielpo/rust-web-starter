use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
}

/// wrapper for http 200 json response
pub fn http_ok(payload: impl Serialize) -> HttpResponse {
    HttpResponse::Ok().json(payload)
}

/// wrapper for http 204 response
pub fn http_no_content() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

/// wrapper for http 400 json response
pub fn http_bad_request(message: String) -> HttpResponse {
    HttpResponse::BadRequest().json(ErrorResponse { message: message })
}

/// wrapper for http 500 json response
pub fn http_internal_server_error(message: String) -> HttpResponse {
    HttpResponse::InternalServerError().json(ErrorResponse { message: message })
}
