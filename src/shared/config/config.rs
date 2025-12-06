use handlebars::{DirectorySourceOptions, Handlebars};
use log::debug;
use mongodb::Client;
use std::env;

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_ADDRESS: &str = "0.0.0.0";
const DEFAULT_TEMPLATES_DIR: &str = "./templates";
const DEFAULT_ASSETS_DIR: &str = "./assets";

/// Mongodb database name
pub const DATABASE_NAME: &str = "template";

pub struct ServerBind {
    pub addr: String,
    pub port: u16,
}

/// Init logger with env variable
pub fn init_logger() {
    /* init logging library */
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
}

/// Init build server bind with env variables
pub fn build_server_bind() -> ServerBind {
    /* init server bind */
    let addr = match env::var("BIND_ADDR") {
        Ok(v) => v,
        Err(_) => DEFAULT_ADDRESS.into(),
    };
    let port = match env::var("BIND_PORT") {
        Ok(v) => v.parse::<u16>().unwrap_or(DEFAULT_PORT),
        Err(_) => DEFAULT_PORT,
    };

    return ServerBind { addr, port };
}

/// init connection to mongodb and return the client
pub async fn init_mongodb() -> Client {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    Client::with_uri_str(uri).await.expect("failed to connect")
}

/// Build handlebars template engine with templates directory
pub fn build_handlebars() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();

    let templates_dir = env::var("TEMPLATES_DIR").unwrap_or_else(|_| {
        let mut path = env::current_dir().expect("Failed to get current directory");
        path.push(DEFAULT_TEMPLATES_DIR);
        path.to_string_lossy().to_string()
    });

    debug!("Loading templates from: {}", templates_dir);

    handlebars
        .register_templates_directory(&templates_dir, DirectorySourceOptions::default())
        .expect("templates directory not found");

    handlebars
}

/// Get assets directory path from env or default
pub fn get_assets_dir() -> String {
    let assets_dir = env::var("ASSETS_DIR").unwrap_or_else(|_| {
        let mut path = env::current_dir().expect("Failed to get current directory");
        path.push(DEFAULT_ASSETS_DIR);
        path.to_string_lossy().to_string()
    });

    debug!("Serving static files from: {}", assets_dir);

    assets_dir
}
