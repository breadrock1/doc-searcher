use getset::Getters;
use serde_derive::Deserialize;
use tracing_loki::url::Url;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const LOKI_SERVICE_NAME: &str = "doc-searcher";

#[derive(Clone, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct LoggerConfig {
    use_loki: bool,
    level: String,
    address: String,
}

pub fn init_logger(config: &LoggerConfig) -> anyhow::Result<()> {
    init_rust_log_env(config);

    let env_filter = tracing_subscriber::EnvFilter::builder().from_env()?;
    if !config.use_loki() {
        tracing_subscriber::FmtSubscriber::builder()
            .with_level(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_env_filter(env_filter)
            .init();
    } else {
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_level(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .pretty();

        let (loki_layer, loki_bg_task) = tracing_loki::builder()
            .label("service", LOKI_SERVICE_NAME)?
            .build_url(Url::parse(config.address())?)?;

        tokio::spawn(loki_bg_task);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(loki_layer)
            .with(fmt_layer)
            .init();
    }

    Ok(())
}

fn init_rust_log_env(config: &LoggerConfig) {
    let level = config.level();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", level);
    }
}
