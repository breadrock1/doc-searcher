use config::{Config, ConfigError, Environment, File, FileFormat};
use derive_builder::Builder;
use dotenv::dotenv;
use gset::Getset;
use serde_derive::Deserialize;

use crate::infrastructure::config::{CacherConfig, StorageConfig};
use crate::infrastructure::httpserver::ServerConfig;
use crate::tracer::{LoggerConfig, TracingConfig};

const CONFIG_PREFIX: &str = "DOC_SEARCHER";
const SERVICE_RUN_MODE: &str = "DOC_SEARCHER__RUN_MODE";
const DEV_FILE_CONFIG_PATH: &str = "./config/development.toml";

#[derive(Builder, Clone, Deserialize, Getset)]
pub struct ServiceConfig {
    #[getset(get, vis = "pub")]
    logger: LoggerConfig,
    #[getset(get, vis = "pub")]
    tracing: TracingConfig,
    #[getset(get, vis = "pub")]
    server: ServerConfig,
    #[getset(get, vis = "pub")]
    storage: StorageConfig,
    #[getset(get, vis = "pub")]
    cacher: CacherConfig,
}

impl ServiceConfig {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();

        let dev_file_config = File::with_name(DEV_FILE_CONFIG_PATH);

        let run_mode = std::env::var(SERVICE_RUN_MODE).unwrap_or("development".into());
        let run_mode_file_path = format!("./config/{run_mode}");
        let file_config = File::with_name(&run_mode_file_path)
            .format(FileFormat::Toml)
            .required(false);

        let env_config = Environment::with_prefix(CONFIG_PREFIX)
            .prefix_separator("__")
            .separator("__")
            .try_parsing(true);

        let settings = Config::builder()
            .add_source(dev_file_config)
            .add_source(file_config)
            .add_source(env_config)
            .build()?;

        settings.try_deserialize()
    }
}
