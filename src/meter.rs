use gset::Getset;
use metrics::{describe_counter, describe_gauge, describe_histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::sync::Arc;

use crate::SERVICE_NAME;

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
            "Count all http requests with status",
        );
        describe_histogram!(
            "http_request_duration_seconds",
            "Store http request processing latency",
        );

        describe_counter!(
            "docsearch_searching_operations_total",
            "Count all searching operations",
        );

        describe_histogram!(
            "docsearch_searching_duration_seconds",
            "Store searching latency",
        );

        describe_counter!(
            "docsearch_storing_operations_total",
            "Count all storing operations with status",
        );

        describe_histogram!(
            "docsearch_storing_duration_seconds",
            "Store latency of stored document",
        );

        describe_counter!(
            "docsearch_paginating_operations_total",
            "Count all paginating operations with status",
        );
        describe_histogram!(
            "docsearch_paginating_duration_seconds",
            "Store paginating latency",
        );

        describe_counter!(
            "docsearch_opensearch_requests_total",
            "Count all outgoing requests to OpenSearch with operation and status",
        );
        describe_histogram!(
            "docsearch_opensearch_request_duration_seconds",
            "Store outgoing OpenSearch request latency",
        );

        describe_counter!(
            "docsearch_cache_operations_total",
            "Count cache operations by status (hit/miss)",
        );

        describe_gauge!(
            "docsearch_http_in_flight_requests",
            "Number of HTTP requests currently being processed",
        );
        describe_histogram!(
            "docsearch_http_request_size_bytes",
            "Size of HTTP request bodies in bytes",
        );
        describe_histogram!(
            "docsearch_http_response_size_bytes",
            "Size of HTTP response bodies in bytes",
        );

        Ok(Arc::new(AppMeterRegistry { meter_handle }))
    }

    pub fn build_local_meter_register() -> anyhow::Result<Arc<AppMeterRegistry>> {
        let meter_handle = PrometheusBuilder::new().build_recorder().handle();

        Ok(Arc::new(AppMeterRegistry { meter_handle }))
    }

    pub fn render_collected_data(&self) -> (&str, String) {
        (RETURNED_FORMAT_TYPE, self.meter_handle.render())
    }
}
