use gset::Getset;
use opentelemetry_sdk::trace::SdkTracerProvider;
use serde_derive::Deserialize;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;

use crate::config::ServiceConfig;

#[derive(Getset, Default)]
pub struct OtlpGuard {
    #[getset(set)]
    tracing_provider: Option<SdkTracerProvider>,
}

impl Drop for OtlpGuard {
    fn drop(&mut self) {
        if let Some(provider) = self.tracing_provider.as_mut() {
            if let Err(err) = provider.shutdown() {
                tracing::error!(err=?err, "failed to shutdown tracing provider");
            }
        }
    }
}

#[derive(Clone, Deserialize, Getset)]
pub struct LoggerConfig {
    #[getset(get_copy, vis = "pub")]
    use_loki: bool,
    #[getset(get, vis = "pub")]
    level: String,
    #[getset(get, vis = "pub")]
    address: String,
}

#[derive(Clone, Deserialize, Getset)]
pub struct TracingConfig {
    #[getset(get_copy, vis = "pub")]
    use_jaeger: bool,
    #[getset(get, vis = "pub")]
    address: String,
}

#[allow(unused_mut)]
pub fn init_otlp_tracing(config: &ServiceConfig) -> anyhow::Result<OtlpGuard> {
    let mut otlp_guard = OtlpGuard::default();

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

    #[cfg(feature = "enable-loki-logger")]
    let logger_layer = init_loki_logger(config.logger())?;
    #[cfg(feature = "enable-loki-logger")]
    let subscriber = subscriber.with(logger_layer);

    #[cfg(feature = "enable-jaeger-tracing")]
    let telemetry = {
        use opentelemetry::global;
        use opentelemetry::trace::TracerProvider;
        use opentelemetry_sdk::propagation::TraceContextPropagator;

        global::set_text_map_propagator(TraceContextPropagator::new());

        let provider = init_jaeger_tracing(config.tracing())?;
        let tracer = provider.tracer(crate::SERVICE_NAME);
        let telemetry = tracing_opentelemetry::layer()
            .with_tracer(tracer)
            .with_location(true)
            .with_threads(true)
            .with_level(true);

        otlp_guard.set_tracing_provider(Some(provider));
        telemetry
    };
    #[cfg(feature = "enable-jaeger-tracing")]
    let subscriber = subscriber.with(telemetry);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(otlp_guard)
}

#[cfg(feature = "enable-jaeger-tracing")]
fn init_jaeger_tracing(config: &TracingConfig) -> anyhow::Result<SdkTracerProvider> {
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_sdk::Resource;

    let resource = Resource::builder()
        .with_service_name(crate::SERVICE_NAME)
        .build();

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

#[cfg(feature = "enable-loki-logger")]
fn init_loki_logger(config: &LoggerConfig) -> anyhow::Result<tracing_loki::Layer> {
    let loki_address = config.address().as_str();
    let loki_url = tracing_loki::url::Url::parse(loki_address)?;
    let (loki_layer, bg_task) = tracing_loki::builder()
        .label("service", crate::SERVICE_NAME)?
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
