use config::{Config, ConfigError, Environment, File, FileFormat};
use derive_builder::Builder;
use dotenv::dotenv;
use getset::Getters;
use serde_derive::Deserialize;

use crate::cacher::config::CacherConfig;
use crate::engine::elastic::config::ElasticConfig;
use crate::logger::LoggerConfig;
use crate::server::config::ServerConfig;
use crate::tokenizer::config::TokenizerConfig;

const CONFIG_PREFIX: &str = "DOC_SEARCHER";
const SERVICE_RUN_MODE: &str = "DOC_SEARCHER__RUN_MODE";
const DEV_FILE_CONFIG_PATH: &str = "./config/development.toml";

#[derive(Builder, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct ServiceConfig {
    logger: LoggerConfig,
    server: ServerConfig,
    elastic: ElasticConfig,
    cacher: CacherConfig,
    tokenizer: TokenizerConfig,
}

impl ServiceConfig {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();

        let dev_file_config = File::with_name(DEV_FILE_CONFIG_PATH);

        let run_mode = std::env::var(SERVICE_RUN_MODE).unwrap_or("development".into());
        let run_mode_file_path = format!("./config/{}", run_mode);
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
