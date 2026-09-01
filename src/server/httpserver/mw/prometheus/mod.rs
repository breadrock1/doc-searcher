use axum::extract::{MatchedPath, Request};
use axum::http::header::CONTENT_LENGTH;
use axum::middleware::Next;
use axum::response::Response;
use metrics::{counter, gauge, histogram};

pub async fn meter(request: Request, next: Next) -> Response {
    let method = request.method().to_string();
    let uri_path = match request.extensions().get::<MatchedPath>() {
        Some(res) => res.as_str().to_owned(),
        None => request.uri().path().to_owned(),
    };

    record_body_size("docsearch_http_request_size_bytes", request.headers());

    let instant = std::time::Instant::now();
    gauge!("docsearch_http_in_flight_requests").increment(1.0);

    // Execute the request
    let response = next.run(request).await;
    gauge!("docsearch_http_in_flight_requests").decrement(1.0);

    let latency = instant.elapsed().as_secs_f64();
    let status = response.status().to_string();

    record_body_size("docsearch_http_response_size_bytes", response.headers());

    let labels = [
        ("uri_path", uri_path),
        ("method", method),
        ("status", status),
    ];

    counter!("http_requests_counter", &labels).increment(1);
    histogram!("http_request_duration_seconds", &labels).record(latency);

    response
}

fn record_body_size(metric_name: &'static str, headers: &axum::http::HeaderMap) {
    if let Some(len) = headers.get(CONTENT_LENGTH) {
        if let Some(v) = len.to_str().ok().and_then(|s| s.parse::<f64>().ok()) {
            histogram!(metric_name).record(v);
        }
    }
}
