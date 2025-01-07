use crate::cacher::config::CacherConfig;
use crate::cors::CorsConfig;
use crate::elastic::ElasticConfig;
use crate::embeddings::config::EmbeddingsConfig;
use crate::logger::LoggerConfig;

use config::{Config, ConfigError, Environment, File};
use derive_builder::Builder;
use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;

const CONFIG_PREFIX: &str = "DOC_SEARCHER";
const SERVICE_RUN_MODE: &str = "DOC_SEARCHER_RUN_MODE";

#[derive(Builder, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct ServiceConfig {
    logger: LoggerConfig,
    server: ServerConfig,
    cors: CorsConfig,
    elastic: ElasticConfig,
    cacher: CacherConfig,
    embeddings: EmbeddingsConfig,
}

#[derive(Clone, Deserialize, CopyGetters, Getters)]
pub struct ServerConfig {
    #[getset(get = "pub")]
    address: String,
    #[getset(get_copy = "pub")]
    port: u16,
    #[getset(get_copy = "pub")]
    workers_num: usize,
}

impl ServiceConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = std::env::var(SERVICE_RUN_MODE).unwrap_or("development".into());

        let run_mode_file_path = format!("./config/{}", run_mode);
        let current_config_file = File::with_name(&run_mode_file_path);

        let settings = Config::builder()
            .add_source(File::with_name("./config/development"))
            .add_source(current_config_file.required(false))
            .add_source(Environment::with_prefix(CONFIG_PREFIX))
            .build()?;

        settings.try_deserialize()
    }
}
