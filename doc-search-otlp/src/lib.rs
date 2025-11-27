mod config;
pub use config::TelemetryConfig;

mod filter;
pub use filter::PathFilter;

use gset::Getset;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::SdkTracerProvider;
use std::str::FromStr;
use tracing_subscriber::Layer;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;

#[derive(Default, Getset)]
pub struct TelemetryGuard {
    #[getset(set, vis = "pub")]
    log_provider: Option<SdkLoggerProvider>,
    #[getset(set, vis = "pub")]
    meter_provider: Option<SdkMeterProvider>,
    #[getset(set, vis = "pub")]
    tracing_provider: Option<SdkTracerProvider>,
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if let Some(meter_provider) = self.meter_provider.as_mut()
            && let Err(err) = meter_provider.shutdown()
        {
            tracing::error!(?err, "failed to shutdown meter provider");
        }

        if let Some(provider) = self.tracing_provider.as_mut()
            && let Err(err) = provider.shutdown()
        {
            tracing::error!(?err, "failed to shutdown tracing provider");
        }
    }
}

pub fn init_telemetry(
    app_name: &'static str,
    config: &TelemetryConfig,
) -> anyhow::Result<TelemetryGuard> {
    init_rust_log_env(config.level());

    let mut telemetry_guard = TelemetryGuard::default();

    let telemetry_layer = match config.enable_remote_otlp() {
        false => None,
        true => {
            let resource = Resource::builder().with_service_name(app_name).build();
            let otlp_addr = config.otlp_address().clone().unwrap_or_default();
            let level =
                LevelFilter::from_str(config.level()).expect("invalid level value into config");

            // Metrics are exported in batch - recommended setup for a production application.
            let metric_exporter = opentelemetry_otlp::MetricExporter::builder()
                .with_tonic()
                .with_endpoint(&otlp_addr)
                .build()
                .expect("Failed to build the metric exporter");
            let meter_provider = SdkMeterProvider::builder()
                .with_periodic_exporter(metric_exporter)
                .with_resource(resource.clone())
                .build();
            telemetry_guard.set_meter_provider(Some(meter_provider.clone()));
            global::set_meter_provider(meter_provider);

            // TODO: Find solution to send logs through tracing
            // Logs are exported in batch - recommended setup for a production application.
            // let log_exporter = LogExporter::builder()
            //     .with_tonic()
            //     .with_endpoint(&otlp_addr)
            //     .with_timeout(Duration::from_secs(5))
            //     .build()
            //     .expect("failed to build the log exporter");
            // let log_provider = SdkLoggerProvider::builder()
            //     .with_batch_exporter(log_exporter)
            //     .with_resource(resource.clone())
            //     .build();
            // telemetry_guard.set_log_provider(Some(log_provider.clone()));

            // Spans are exported in batch - recommended setup for a production application.
            global::set_text_map_propagator(TraceContextPropagator::new());
            let span_exporter = opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .with_endpoint(&otlp_addr)
                .build()
                .expect("failed to build the span exporter");
            let tracer_provider = SdkTracerProvider::builder()
                .with_batch_exporter(span_exporter)
                .with_resource(resource.clone())
                .build();
            telemetry_guard.set_tracing_provider(Some(tracer_provider.clone()));

            let tracer = tracer_provider.tracer(app_name);
            let telemetry_layer = tracing_opentelemetry::layer()
                .with_tracer(tracer)
                .with_filter(level);

            Some(telemetry_layer)
        }
    };

    let loki_layer = {
        match config.enable_direct_loki() {
            false => None,
            true => {
                let address = config
                    .loki_address()
                    .as_ref()
                    .expect("missing loki address into config");

                let loki_url = tracing_loki::url::Url::parse(address)
                    .expect("failed to parse loki url address");

                let (loki_layer, bg_task) = tracing_loki::builder()
                    .label("service", app_name)
                    .expect("failed to set service label")
                    .build_url(loki_url)
                    .expect("failed to build loki url");

                tokio::spawn(bg_task);

                Some(loki_layer)
            }
        }
    };

    let env_filter = tracing_subscriber::EnvFilter::from_default_env();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_span_events(FmtSpan::NEW)
        .pretty();

    let common_subscriber = tracing_subscriber::registry()
        .with(fmt_layer)
        .with(env_filter)
        .with(telemetry_layer)
        .with(loki_layer);

    tracing::subscriber::set_global_default(common_subscriber)?;

    Ok(telemetry_guard)
}

fn init_rust_log_env(level: &String) {
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", level);
        }
    }
}
