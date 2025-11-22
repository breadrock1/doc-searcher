use axum::http::Request;
use regex::Regex;
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Clone)]
pub struct PathFilter {
    paths: Vec<Regex>,
}

impl Default for PathFilter {
    fn default() -> Self {
        PathFilter {
            paths: vec![
                Regex::new("/health").unwrap(),
                Regex::new("/metrics").unwrap(),
                Regex::new("/api/.*/swagger").unwrap(),
            ],
        }
    }
}

struct HeaderExtractor<'a>(&'a axum::http::HeaderMap);

impl<'a> opentelemetry::propagation::Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| value.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}

impl<B> tower_http::trace::MakeSpan<B> for PathFilter {
    fn make_span(&mut self, request: &Request<B>) -> tracing::Span {
        let path = request.uri().path();
        if self.is_path_ignored(path) {
            return tracing::span!(tracing::Level::DEBUG, "http-request-filtered");
        }

        let span = tracing::info_span!(
            "http-request-parent",
            method = %request.method(),
            status_code = tracing::field::Empty,
            uri = %request.uri(),
            version = ?request.version(),
        );

        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&HeaderExtractor(request.headers()))
        });

        span.set_parent(parent_context);
        span
    }
}

impl PathFilter {
    pub fn is_path_ignored(&self, path: &str) -> bool {
        self.paths.iter().any(|it| it.is_match_at(path, 0))
    }
}
