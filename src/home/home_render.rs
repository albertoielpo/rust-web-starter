use actix_web::{HttpResponse, Result, get, web};
use handlebars::Handlebars;
use redis::{AsyncCommands, RedisError, aio::ConnectionManager};
use time::{OffsetDateTime, format_description::well_known::Iso8601};

use crate::{home::dto::HomeData, shared::config::settings::RedisKeys};

/// Serves the home page by rendering the Handlebars template.
///
/// This handler loads the `home.hbs` template
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
async fn home(
    hb: web::Data<Handlebars<'_>>,
    redis: web::Data<ConnectionManager>,
) -> Result<HttpResponse> {
    let now = OffsetDateTime::now_utc();
    // find data in redis
    let res: Result<String, RedisError> = redis
        .get_ref()
        .clone()
        .get(RedisKeys::FirstHit.as_str())
        .await;

    let iso_date = match res {
        Ok(res) => res, // if found
        Err(_) => {
            let iso_date = now
                .format(&Iso8601::DEFAULT)
                .map_err(actix_web::error::ErrorInternalServerError)?;
            let res: Result<String, RedisError> = redis
                .get_ref()
                .clone()
                .set(RedisKeys::FirstHit.as_str(), iso_date.as_str())
                .await;
            res.map_err(actix_web::error::ErrorInternalServerError)?;
            iso_date
        }
    };

    let data = HomeData {
        first_hit: iso_date,
        title: "Rust web starter".to_owned(),
    };
    let body = hb
        .render("home", &data)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

/// Service configuration for home page routes.
///
/// Registers all home page handlers with the Actix-web application.
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(home);
}
