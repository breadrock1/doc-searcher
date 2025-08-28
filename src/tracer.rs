use gset::Getset;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use serde_derive::Deserialize;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::ServiceConfig;
use crate::SERVICE_NAME;

#[derive(Getset)]
pub struct OtlpGuard {
    #[getset(set)]
    tracing_provider: Option<SdkTracerProvider>
}

impl Default for OtlpGuard {
    fn default() -> Self {
        OtlpGuard { tracing_provider: None }
    }
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

pub fn init_otlp_tracing(config: &ServiceConfig) -> anyhow::Result<OtlpGuard> {
    let mut otlp_guard = OtlpGuard::default();

    init_rust_log_env(config.logger());
    let env_filter = tracing_subscriber::EnvFilter::from_default_env();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_span_events(FmtSpan::FULL)
        .pretty();

    global::set_text_map_propagator(TraceContextPropagator::new());
    let provider = init_jaeger_tracing(config.tracing())?;
    let tracer = provider.tracer(SERVICE_NAME);
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let loki_address = config.logger().address().as_str();
    let loki_url = tracing_loki::url::Url::parse(loki_address)?;
    let (loki_layer, bg_task) = tracing_loki::builder()
        .label("service", SERVICE_NAME)?
        .build_url(loki_url)?;
    tokio::spawn(bg_task);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .with(loki_layer)
        .with(telemetry)
        .init();

    otlp_guard.set_tracing_provider(Some(provider));

    Ok(otlp_guard)
}

fn init_jaeger_tracing(config: &TracingConfig) -> anyhow::Result<SdkTracerProvider> {
    let resource = Resource::builder()
        .with_service_name(SERVICE_NAME)
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

fn init_rust_log_env(config: &LoggerConfig) {
    let level = config.level();
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", level);
        }
    }
}
