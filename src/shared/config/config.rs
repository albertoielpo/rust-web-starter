use mongodb::Client;
use std::env;

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_ADDRESS: &str = "0.0.0.0";

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
