use handlebars::{DirectorySourceOptions, Handlebars};
use log::{debug, error, info};
use mongodb::{options::ClientOptions, Client};
use std::{env, time::Duration};

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_ADDRESS: &str = "0.0.0.0";
const DEFAULT_TEMPLATES_DIR: &str = "./templates";
const DEFAULT_ASSETS_DIR: &str = "./assets";
const DEFAULT_MONGODB_TIMEOUT_SECS: u64 = 10;

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
    let uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let timeout_secs = env::var("MONGODB_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(DEFAULT_MONGODB_TIMEOUT_SECS);

    debug!(
        "Connecting to MongoDB at: {} (timeout: {}s)",
        uri, timeout_secs
    );

    let mut client_options = match ClientOptions::parse(&uri).await {
        Ok(opts) => opts,
        Err(e) => {
            error!("Failed to parse MongoDB URI {}: {}", uri, e);
            panic!("Failed to parse MongoDB URI: {}", e);
        }
    };

    // Set connection timeout
    client_options.connect_timeout = Some(Duration::from_secs(timeout_secs));
    client_options.server_selection_timeout = Some(Duration::from_secs(timeout_secs));

    let client = match Client::with_options(client_options) {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create MongoDB client for {}: {}", uri, e);
            panic!("Failed to create MongoDB client: {}", e);
        }
    };

    // Verify connection by pinging the database
    debug!("Verifying MongoDB connection with ping...");
    match client
        .database("admin")
        .run_command(mongodb::bson::doc! { "ping": 1 })
        .await
    {
        Ok(_) => {
            info!("Successfully connected to MongoDB");
            client
        }
        Err(e) => {
            error!("Failed to connect to MongoDB at {}: {}", uri, e);
            panic!("Failed to connect to MongoDB: {}", e);
        }
    }
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
