use gset::Getset;
use serde_derive::Deserialize;

#[derive(Clone, Deserialize, Getset)]
pub struct TelemetryConfig {
    #[getset(get, vis = "pub")]
    level: String,
    #[getset(get_copy, vis = "pub")]
    enable_remote_otlp: bool,
    #[getset(get, vis = "pub")]
    otlp_address: Option<String>,
}

// #[derive(Clone, Deserialize, Getset)]
// pub struct LoggerConfig {
//     #[getset(get, vis = "pub")]
//     level: String,
//     #[getset(get, vis = "pub")]
//     address: String,
// }
//
// #[derive(Clone, Deserialize, Getset)]
// pub struct TracingConfig {
//     #[getset(get, vis = "pub")]
//     address: String,
// }
