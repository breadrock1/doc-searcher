mod config;
pub use config::OtlpConfig;

mod filter;
pub use filter::PathFilter;

use gset::Getset;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;

use crate::config::{LoggerConfig, TracingConfig};

#[derive(Getset, Default)]
pub struct OtlpStateGuard {
    #[getset(set)]
    tracing_provider: Option<SdkTracerProvider>,
}

impl Drop for OtlpStateGuard {
    fn drop(&mut self) {
        if let Some(provider) = self.tracing_provider.as_mut()
            && let Err(err) = provider.shutdown()
        {
            tracing::error!(err=?err, "failed to shutdown tracing provider");
        }
    }
}

pub fn init_otlp_tracing(
    app_name: &'static str,
    config: &OtlpConfig,
) -> anyhow::Result<OtlpStateGuard> {
    if config.enable_remote_otlp() {
        return init_remote_otlp_tracing(app_name, config);
    }

    init_local_otlp_tracing(config)
}

fn init_local_otlp_tracing(config: &OtlpConfig) -> anyhow::Result<OtlpStateGuard> {
    let otlp_guard = OtlpStateGuard::default();

    init_rust_log_env(config.logger());
    let env_filter = tracing_subscriber::EnvFilter::from_default_env();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_span_events(FmtSpan::NEW)
        .pretty();

    let subscriber = tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(otlp_guard)
}

fn init_remote_otlp_tracing(
    app_name: &'static str,
    config: &OtlpConfig,
) -> anyhow::Result<OtlpStateGuard> {
    let mut otlp_guard = OtlpStateGuard::default();

    init_rust_log_env(config.logger());
    let env_filter = tracing_subscriber::EnvFilter::from_default_env();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_span_events(FmtSpan::NEW)
        .pretty();

    let subscriber = tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(fmt_layer);

    let logger_layer = init_loki_logger(app_name, config.logger())?;
    let subscriber = subscriber.with(logger_layer);

    let telemetry = {
        use opentelemetry::global;
        use opentelemetry::trace::TracerProvider;
        use opentelemetry_sdk::propagation::TraceContextPropagator;

        global::set_text_map_propagator(TraceContextPropagator::new());

        let provider = init_jaeger_tracing(app_name, config.tracing())?;
        let tracer = provider.tracer(app_name);
        let telemetry = tracing_opentelemetry::layer()
            .with_tracer(tracer)
            .with_location(true)
            .with_threads(true)
            .with_level(true);

        otlp_guard.set_tracing_provider(Some(provider));
        telemetry
    };
    let subscriber = subscriber.with(telemetry);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(otlp_guard)
}

fn init_jaeger_tracing(
    app_name: &'static str,
    config: &TracingConfig,
) -> anyhow::Result<SdkTracerProvider> {
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_sdk::Resource;

    let resource = Resource::builder().with_service_name(app_name).build();

    let jaeger_endpoint = format!("{}/api/traces", config.address());
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(jaeger_endpoint)
        .build()?;

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter)
        .with_resource(resource)
        .build();

    Ok(provider)
}

fn init_loki_logger(
    app_name: &'static str,
    config: &LoggerConfig,
) -> anyhow::Result<tracing_loki::Layer> {
    let loki_address = config.address().as_str();
    let loki_url = tracing_loki::url::Url::parse(loki_address)?;
    let (loki_layer, bg_task) = tracing_loki::builder()
        .label("service", app_name)?
        .build_url(loki_url)?;

    tokio::spawn(bg_task);

    Ok(loki_layer)
}

fn init_rust_log_env(config: &LoggerConfig) {
    let level = config.level();
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", level);
        }
    }
}
