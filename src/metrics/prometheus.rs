use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use std::collections::HashMap;

pub fn init_prometheus() -> Result<PrometheusMetrics, anyhow::Error> {
    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), "value1".to_string());
    let prometheus = PrometheusMetricsBuilder::new("metrics")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .map_err(|err| anyhow::Error::msg(err.to_string()))?;

    Ok(prometheus)
}
