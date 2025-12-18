use axum::body::{Body, Bytes};
use axum::extract::{MatchedPath, Request, State};
use axum::middleware::Next;
use axum::response::Response;
use metrics::{counter, histogram};

pub async fn meter(request: Request, next: Next) -> Response {
    let method = request.method().to_string();
    let uri_path = match request.extensions().get::<MatchedPath>() {
        Some(res) => res.as_str().to_owned(),
        None => request.uri().path().to_owned(),
    };

    let instant = std::time::Instant::now();

    // Execute the request
    let response = next.run(request).await;
    let latency = instant.elapsed().as_secs_f64();
    let status = response.status().to_string();

    let labels = [
        ("uri_path", uri_path),
        ("method", method),
        ("status", status),
    ];

    counter!("http_requests_counter", &labels).increment(1);
    histogram!("http_request_duration_seconds", &labels).record(latency);

    response
}
