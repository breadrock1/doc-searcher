use gset::Getset;
use serde_derive::Deserialize;

#[derive(Clone, Deserialize, Getset)]
pub struct OtlpConfig {
    #[getset(get_copy, vis = "pub")]
    enable_remote_otlp: bool,
    #[getset(get, vis = "pub")]
    logger: LoggerConfig,
    #[getset(get, vis = "pub")]
    tracing: TracingConfig,
}

#[derive(Clone, Deserialize, Getset)]
pub struct LoggerConfig {
    #[getset(get, vis = "pub")]
    level: String,
    #[getset(get, vis = "pub")]
    address: String,
}

#[derive(Clone, Deserialize, Getset)]
pub struct TracingConfig {
    #[getset(get, vis = "pub")]
    address: String,
}
