use axum::body::{Body, Bytes};
use axum::extract::{Request, State};
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::Response;
use std::sync::Arc;
use tower_http::add_extension::AddExtensionLayer;

use crate::application::services::cacher::Cacher;
use crate::infrastructure::redis::config::RedisConfig;
use crate::infrastructure::redis::RedisClient;
use crate::ServiceConnect;

struct CacheState {
    client: Arc<RedisClient>,
    filters: Vec<regex::Regex>,
}

impl CacheState {
    fn new(client: Arc<RedisClient>, filters: Vec<regex::Regex>) -> Self {
        CacheState { client, filters }
    }
}

pub async fn enable_caching_mw(
    app: axum::Router,
    config: &RedisConfig,
) -> anyhow::Result<axum::Router> {
    let filters = ["/search/paginate/*"]
        .into_iter()
        .filter_map(|it| regex::Regex::new(it).ok())
        .collect::<Vec<regex::Regex>>();

    let redis = RedisClient::connect(config).await?;
    let cache_state = CacheState::new(Arc::new(redis), filters);
    let state_arc = Arc::new(cache_state);

    let ext_layer = AddExtensionLayer::new(state_arc.clone());
    let cache_mw = axum::middleware::from_fn_with_state(state_arc, cache);
    let tower_layer = tower::ServiceBuilder::new()
        .layer(ext_layer)
        .layer(cache_mw);

    Ok(app.layer(tower_layer))
}

async fn cache(State(cache): State<Arc<CacheState>>, request: Request, next: Next) -> Response {
    let path = request.uri().path();
    let is_matched_path = cache
        .filters
        .iter()
        .map(|it| it.is_match_at(path, 0))
        .any(|it| it == true);

    if !is_matched_path {
        return next.run(request).await;
    }

    let header_str = headers_to_key(request.headers());
    let cache_key = format!("{path}:{header_str}");
    let cached: Option<Vec<u8>> = cache.client.load(&cache_key).await;
    if let Some(value) = cached {
        if !value.is_empty() {
            let data = Bytes::from(value);
            return Response::new(Body::from(data));
        }
    }

    // Execute the request
    let response = next.run(request).await;
    match response.status().is_success() {
        true => {
            let body = response.into_body();
            let data = axum::body::to_bytes(body, usize::MAX).await.unwrap();
            let data_vec = data.to_vec();
            cache.client.store(&cache_key, &data_vec).await;
            Response::new(Body::from(data))
        }
        false => response,
    }
}

const NULL_HEADER_VALUE: &str = "null";
const HEADER_FIELDS: [&str; 2] = ["Accept", "Authorization"];

fn headers_to_key(headers: &HeaderMap) -> String {
    HEADER_FIELDS
        .into_iter()
        .map(|it| match headers.get(it) {
            None => NULL_HEADER_VALUE,
            Some(value) => value.to_str().unwrap_or(NULL_HEADER_VALUE),
        })
        .collect::<Vec<&str>>()
        .join(":")
}
