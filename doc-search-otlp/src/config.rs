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
    #[getset(get_copy, vis = "pub")]
    enable_direct_loki: bool,
    #[getset(get, vis = "pub")]
    loki_address: Option<String>,
}
