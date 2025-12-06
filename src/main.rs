//! Architecture model application built with Actix-web and Handlebars.
//!
//! This application demonstrates a simple web server using Actix-web framework
//! with Handlebars templating for server-side rendering.
//!
//! # Author
//! Alberto Ielpo
//!
//! # License
//! MIT
use actix_files::Files;
use actix_web::{
    App, HttpResponse, HttpServer, Result,
    middleware::{Logger, NormalizePath, TrailingSlash},
    web,
};
use actix_web_lab::middleware::CatchPanic;
use handlebars::Handlebars;
use log::debug;
use redis::{AsyncCommands, RedisError, aio::ConnectionManager};
use rust_web_starter::{
    shared::config::config::{
        build_handlebars, build_server_bind, get_assets_dir, init_logger, init_mongodb, init_redis,
    },
    users,
};
use serde::Serialize;
use time::{OffsetDateTime, format_description::well_known::Iso8601};

#[derive(Serialize)]
struct IndexData {
    iso_date: String,
    mykey: String,
}

/// Serves the index page by rendering the Handlebars template.
///
/// This handler loads the `index.hbs` template, injects the current ISO 8601
/// formatted timestamp, and returns the rendered HTML response.
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

/// Application entry point.
///
/// Initializes the Actix-web server, configures Handlebars templating,
/// and starts listening for HTTP requests on 127.0.0.1:8080.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    let handlebars = build_handlebars();
    let assets_dir = get_assets_dir();
    let server_bind = build_server_bind();
    let mongodb_client = init_mongodb().await;
    let redis_manager = init_redis().await;

    let handlebars_ref = web::Data::new(handlebars);
    let mongodb_ref = web::Data::new(mongodb_client);
    let redis_ref = web::Data::new(redis_manager);

    debug!(
        "Server bind: address {} port {}",
        server_bind.addr, server_bind.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(mongodb_ref.clone())
            .app_data(redis_ref.clone())
            .app_data(handlebars_ref.clone())
            .wrap(NormalizePath::new(TrailingSlash::Trim)) // normalize path
            .wrap(CatchPanic::default()) // CatchPanic must be before Logger
            .wrap(Logger::default()) // last wrap
            // html page
            .route("/", web::get().to(index))
            // static assets
            .service(Files::new("/assets", assets_dir.clone()))
            // rest controller
            .service(web::scope("/users").configure(users::users_controller::config))
    })
    .bind((server_bind.addr, server_bind.port))?
    .run()
    .await
}
