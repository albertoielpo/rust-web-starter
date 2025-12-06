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
use handlebars::{DirectorySourceOptions, Handlebars};
use log::debug;
use rust_web_starter::{
    shared::config::config::{build_server_bind, init_logger, init_mongodb},
    users,
};
use serde::Serialize;
use std::env;
use time::{OffsetDateTime, format_description::well_known::Iso8601};

#[derive(Serialize)]
struct IndexData {
    iso_date: String,
}

/// Serves the index page by rendering the Handlebars template.
///
/// This handler loads the `index.hbs` template, injects the current ISO 8601
/// formatted timestamp, and returns the rendered HTML response.
async fn index(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse> {
    let now = OffsetDateTime::now_utc();
    let iso_date = now
        .format(&Iso8601::DEFAULT)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let data = IndexData { iso_date };

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
    let mut handlebars = Handlebars::new();

    // Get templates directory from TEMPLATES_DIR environment variable
    // or default to ./templates relative to the current working directory
    let templates_dir = env::var("TEMPLATES_DIR").unwrap_or_else(|_| {
        let mut path = env::current_dir().expect("Failed to get current directory");
        path.push("templates");
        path.to_string_lossy().to_string()
    });

    println!("Loading templates from: {}", templates_dir);

    // Register all Handlebars templates from the templates directory
    handlebars
        .register_templates_directory(&templates_dir, DirectorySourceOptions::default())
        .expect("templates directory not found");

    let handlebars_ref = web::Data::new(handlebars);

    // Get assets directory from ASSETS_DIR environment variable
    // or default to ./assets relative to the current working directory
    let assets_dir = env::var("ASSETS_DIR").unwrap_or_else(|_| {
        let mut path = env::current_dir().expect("Failed to get current directory");
        path.push("assets");
        path.to_string_lossy().to_string()
    });

    println!("Serving static files from: {}", assets_dir);

    init_logger();
    let server_bind = build_server_bind();
    let mongodb_client = init_mongodb().await;

    debug!(
        "Server bind: address {} port {}",
        server_bind.addr, server_bind.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mongodb_client.clone()))
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
