//! Rust web starter application.
//!
//! A production-ready web server starter template featuring:
//! - **Actix-web** framework for high-performance HTTP handling
//! - **Handlebars** templating engine for server-side rendering
//! - **MongoDB** integration for document storage
//! - **Redis** integration for caching and session management
//! - Structured logging with configurable levels
//! - Static file serving
//! - RESTful API endpoints
//!
//! # Architecture
//!
//! The application follows a modular architecture with:
//! - Middleware stack: panic handling, path normalization, request logging
//! - Dependency injection via Actix-web's `Data` extractor
//! - Separate controller modules for API routes (e.g., users module)
//!
//! # Quick Start
//!
//! ```bash
//! cargo run
//! ```
//!
//! The server listens on the configured address (default: 0.0.0.0:3000).
//!
//! # Author
//! Alberto Ielpo
//!
//! # License
//! MIT
use actix_files::Files;
use actix_web::{
    App, HttpServer,
    middleware::{Logger, NormalizePath, TrailingSlash},
    web,
};
use actix_web_lab::middleware::CatchPanic;
use log::debug;
use rust_web_starter::{
    home,
    shared::config::config::{
        build_handlebars, build_server_bind, get_assets_dir, init_logger, init_mongodb, init_redis,
    },
    users,
};

/// Application entry point.
///
/// Initializes the Actix-web server, configures Handlebars templating,
/// and starts listening for HTTP requests on 0.0.0.0:3000.
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
            // render, response text/html on path /
            .service(web::scope("/").configure(home::home_render::config))
            // static assets, serve as is
            .service(Files::new("/assets", assets_dir.clone()))
            // rest controllers, response application/json on path /users
            .service(web::scope("/users").configure(users::users_controller::config))
    })
    .bind((server_bind.addr, server_bind.port))?
    .run()
    .await
}
