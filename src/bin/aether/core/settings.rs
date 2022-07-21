/*
   Appellation: settings <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::collect_config_files;
pub use components::*;

type ConfigBuilderDS = config::ConfigBuilder<config::builder::DefaultState>;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Configuration {
    pub application: Application,
    pub database: Database,
    pub logger: Logger,
    pub server: Server,
}

impl Configuration {
    pub fn constructor() -> Result<ConfigBuilderDS, config::ConfigError> {
        let mut builder = config::Config::builder()
            .set_default("application.mode", "development")?
            .set_default("application.name", "aether")?
            .set_default("database.name", "postgres")?
            .set_default(
                "database.uri",
                "postgres://postgres:example@localhost:5432/postgres",
            )?
            .set_default("logger.level", "info")?
            .set_default("server.host", "[0, 0, 0, 0]")?
            .set_default("server.port", 8080)?;

        builder = builder.add_source(collect_config_files("**/*.config.*", false));
        builder = builder.add_source(config::Environment::default().separator("__"));
        Ok(builder)
    }
    pub fn new() -> Result<Self, config::ConfigError> {
        Self::constructor().ok().unwrap().build()?.try_deserialize()
    }
}

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Configuration(application={}, database={}, logger={})",
            self.application, self.database, self.logger
        )
    }
}

mod components {
    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Application {
        pub mode: String,
        pub name: String,
    }

    impl std::fmt::Display for Application {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Application(mode={}, name={})", self.mode, self.name)
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Database {
        pub name: String,
        pub uri: String,
    }

    impl std::fmt::Display for Database {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Database(name={}, uri={})", self.name, self.uri)
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Server {
        pub host: String,
        pub port: u16,
    }

    impl std::fmt::Display for Server {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Server(port={})", self.port)
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Logger {
        pub level: String,
    }

    impl Logger {
        pub fn setup(configuration: &crate::Configuration) {
            if std::env::var_os("RUST_LOG").is_none() {
                let level = configuration.logger.level.as_str();
                std::env::set_var("RUST_LOG", level);
            }

            tracing_subscriber::fmt::init();
        }
    }

    impl std::fmt::Display for Logger {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Logger(level={})", self.level)
        }
    }
}
