use getset::Getters;
use serde_derive::Deserialize;
use tracing_subscriber::filter::FromEnvError;

#[derive(Clone, Deserialize, Getters)]
pub struct LoggerConfig {
    #[getset(get = "pub")]
    level: String,
}

pub fn init_logger(config: &LoggerConfig) -> Result<(), FromEnvError> {
    init_rust_log_env(config);

    let env_filter = tracing_subscriber::EnvFilter::builder().from_env()?;
    tracing_subscriber::FmtSubscriber::builder()
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_env_filter(env_filter)
        .init();

    Ok(())
}

fn init_rust_log_env(config: &LoggerConfig) {
    let level = config.level();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", level);
    }
}
