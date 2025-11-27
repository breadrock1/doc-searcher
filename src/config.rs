use config::{Config, ConfigError, Environment, File, FileFormat};
use doc_search_otlp::OtlpConfig;
use dotenv::dotenv;
use gset::Getset;
use serde_derive::Deserialize;

use crate::server::{CacheConfig, ServerConfig, StorageConfig};

const CONFIG_PREFIX: &str = "DOC_SEARCH";
const SERVICE_RUN_MODE: &str = "DOC_SEARCH__RUN_MODE";
const DEV_FILE_CONFIG_PATH: &str = "./config/development.toml";

#[derive(Clone, Deserialize, Getset)]
pub struct ServiceConfig {
    #[getset(get, vis = "pub")]
    settings: SettingsConfig,
    #[getset(get, vis = "pub")]
    otlp: OtlpConfig,
    #[getset(get, vis = "pub")]
    server: ServerConfig,
    #[getset(get, vis = "pub")]
    storage: StorageConfig,
    #[getset(get, vis = "pub")]
    cache: CacheConfig,
}

#[derive(Clone, Deserialize, Getset)]
pub struct SettingsConfig {
    #[getset(get_copy, vis = "pub")]
    max_content_size: usize,
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
