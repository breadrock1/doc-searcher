use crate::cacher::config::CacherConfig;
use crate::cors::CorsConfig;
use crate::elastic::ElasticConfig;
use crate::embeddings::config::EmbeddingsConfig;
use crate::logger::LoggerConfig;

use config::{Config, ConfigError, Environment, File};
use derive_builder::Builder;
use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;

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
        let run_mode = std::env::var("DOC_SEARCHER_RUN_MODE").unwrap_or("development".into());

        let run_mode_file_path = format!("./config/{}", run_mode);
        let file_config = File::with_name(&run_mode_file_path).required(false);

        let env_config = Environment::with_prefix("DOC_SEARCHER").try_parsing(true);

        let settings = Config::builder()
            .add_source(file_config)
            .add_source(env_config)
            .build()?;

        settings.try_deserialize()
    }
}
