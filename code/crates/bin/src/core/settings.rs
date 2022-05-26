use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Application {
    pub mode: String,
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Database {
    pub name: String,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Provider {
    pub endpoint: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server {
    pub host: [u8; 4],
    pub port: u16,
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "View the bin locally at http://localhost:{}", self.port)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub application: Application,
    pub database: Database,
    pub logger: Logger,
    pub provider: Provider,
    pub server: Server
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            .add_source(File::with_name("config/default.config"))
            .add_source(File::with_name(&format!("config/{}.config", run_mode)).required(false))
            .add_source(File::with_name("config/local.config").required(false))
            .add_source(Environment::default().separator("__"));


        if let Ok(port) = std::env::var("PORT") {
            builder = builder.set_override("server.port", port)?;
        }
        builder.build()?.try_deserialize()
    }
}