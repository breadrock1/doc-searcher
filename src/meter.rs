use doc_search_core::SERVICE_NAME;
use gset::Getset;
use metrics::{describe_counter, describe_histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::sync::Arc;

const RETURNED_FORMAT_TYPE: &str = "text/plain";

#[derive(Getset)]
pub struct AppMeterRegistry {
    #[getset(get, vis = "pub")]
    meter_handle: PrometheusHandle,
}

impl AppMeterRegistry {
    pub fn build_meter_registry() -> anyhow::Result<Arc<AppMeterRegistry>> {
        let meter_handle = PrometheusBuilder::new()
            .add_global_label("service", SERVICE_NAME)
            .install_recorder()
            .expect("failed to install Prometheus recorder");

        describe_counter!(
            "http_requests_counter",
            "Count all http requests with status"
        );
        describe_histogram!(
            "http_request_duration_seconds",
            "Store http request processing latency"
        );

        Ok(Arc::new(AppMeterRegistry { meter_handle }))
    }

    pub fn render_collected_data(&self) -> (&str, String) {
        (RETURNED_FORMAT_TYPE, self.meter_handle.render())
    }
}
