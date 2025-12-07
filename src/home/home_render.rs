use super::dto::IndexData;
use actix_web::{HttpResponse, Result, get, web};
use handlebars::Handlebars;
use redis::{AsyncCommands, RedisError, aio::ConnectionManager};
use time::{OffsetDateTime, format_description::well_known::Iso8601};

/// Serves the home page by rendering the Handlebars template.
///
/// This handler loads the `index.hbs` template
///
/// # Route
/// `GET /` - Home page
///
/// # Arguments
/// * `hb` - Handlebars template engine instance
/// * `redis` - Redis connection manager for caching
///
/// # Returns
/// Rendered HTML page or error response
#[get("")]
async fn index(
    hb: web::Data<Handlebars<'_>>,
    redis: web::Data<ConnectionManager>,
) -> Result<HttpResponse> {
    let now = OffsetDateTime::now_utc();
    let iso_date = now
        .format(&Iso8601::DEFAULT)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let x: Result<String, RedisError> = redis.get_ref().clone().get("mykey").await;
    let x = x.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let data = IndexData { iso_date, mykey: x };
    let body = hb
        .render("index", &data)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

/// Service configuration for home page routes.
///
/// Registers all home page handlers with the Actix-web application.
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
