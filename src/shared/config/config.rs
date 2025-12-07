use handlebars::{DirectorySourceOptions, Handlebars};
use log::{debug, error, info};
use mongodb::{Client, options::ClientOptions};
use redis::aio::ConnectionManager;
use std::{env, time::Duration};

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_ADDRESS: &str = "0.0.0.0";
const DEFAULT_TEMPLATES_DIR: &str = "./templates";
const DEFAULT_ASSETS_DIR: &str = "./assets";
const DEFAULT_MONGODB_TIMEOUT_SECS: u64 = 10;
const DEFAULT_REDIS_TIMEOUT_SECS: u64 = 10;

/// MongoDB database name used across the application.
pub const DATABASE_NAME: &str = "template";

pub struct ServerBind {
    pub addr: String,
    pub port: u16,
}

/// Initializes the logger with environment variable configuration.
///
/// Uses `RUST_LOG` environment variable, defaults to `debug` level.
pub fn init_logger() {
    /* init logging library */
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
}

/// Builds server bind configuration from environment variables.
///
/// # Environment Variables
/// - `BIND_ADDR` - Server bind address (default: 0.0.0.0)
/// - `BIND_PORT` - Server port (default: 3000)
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

/// Initializes MongoDB connection and returns the client.
///
/// # Environment Variables
/// - `MONGODB_URI` - MongoDB connection string (default: mongodb://localhost:27017)
/// - `MONGODB_TIMEOUT_SECS` - Connection timeout in seconds (default: 10)
///
/// # Panics
/// Panics if the connection cannot be established within the timeout period.
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

/// Builds Handlebars template engine with templates directory.
///
/// # Environment Variables
/// - `TEMPLATES_DIR` - Path to templates directory (default: ./templates)
///
/// # Panics
/// Panics if the templates directory is not found.
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

/// Gets the assets directory path from environment or default.
///
/// # Environment Variables
/// - `ASSETS_DIR` - Path to static assets directory (default: ./assets)
pub fn get_assets_dir() -> String {
    let assets_dir = env::var("ASSETS_DIR").unwrap_or_else(|_| {
        let mut path = env::current_dir().expect("Failed to get current directory");
        path.push(DEFAULT_ASSETS_DIR);
        path.to_string_lossy().to_string()
    });

    debug!("Serving static files from: {}", assets_dir);

    assets_dir
}

/// Initializes Redis connection and returns the connection manager.
///
/// # Environment Variables
/// - `REDIS_URI` - Redis connection string (default: redis://localhost:6379)
/// - `REDIS_TIMEOUT_SECS` - Connection timeout in seconds (default: 10)
///
/// # Panics
/// Panics if the connection cannot be established within the timeout period.
pub async fn init_redis() -> ConnectionManager {
    let uri = env::var("REDIS_URI").unwrap_or_else(|_| "redis://localhost:6379".into());

    let timeout_secs = env::var("REDIS_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(DEFAULT_REDIS_TIMEOUT_SECS);

    debug!(
        "Connecting to Redis at: {} (timeout: {}s)",
        uri, timeout_secs
    );

    let client = match redis::Client::open(uri.as_str()) {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create Redis client for {}: {}", uri, e);
            panic!("Failed to create Redis client: {}", e);
        }
    };

    // Create connection manager with automatic reconnection (with timeout)
    let timeout_duration = Duration::from_secs(timeout_secs);
    match actix_web::rt::time::timeout(timeout_duration, ConnectionManager::new(client)).await {
        Ok(Ok(manager)) => manager,
        Ok(Err(e)) => {
            error!("Failed to create Redis connection manager: {}", e);
            panic!("Failed to create Redis connection manager: {}", e);
        }
        Err(_) => {
            error!(
                "Timeout creating Redis connection manager after {}s",
                timeout_secs
            );
            panic!(
                "Timeout creating Redis connection manager after {}s",
                timeout_secs
            );
        }
    }
}
