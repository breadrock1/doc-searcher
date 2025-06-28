use axum::body::{Body, Bytes};
use axum::extract::{Request, State};
use axum::http::HeaderMap;
use axum::response::Response;
use axum::middleware::Next;
use std::sync::Arc;
use tower_http::add_extension::AddExtensionLayer;

use crate::application::services::cacher::Cacher;
use crate::infrastructure::redis::config::RedisConfig;
use crate::infrastructure::redis::RedisClient;
use crate::ServiceConnect;

pub async fn enable_cache_mw(app: axum::Router, config: &RedisConfig) -> anyhow::Result<axum::Router> {
    let redis_config = config.cacher().redis();
    let redis = RedisClient::connect(redis_config).await?;
    let redis_arc = Arc::new(redis);

    let ext_layer = AddExtensionLayer::new(redis_arc.clone());
    let cache_mw = axum::middleware::from_fn_with_state(redis_arc, cache_middleware);
    let tower_layer = tower::ServiceBuilder::new()
        .layer(ext_layer)
        .layer(cache_mw);

    Ok(app.layer(tower_layer))
}

async fn cache_middleware(State(cache): State<Arc<RedisClient>>, request: Request, next: Next) -> Response {
    // // Skip caching for non-GET requests
    // if request.method() != "GET" {
    //     return next.run(request).await;
    // }

    let path = request.uri().path().to_string();

    // TODO: Filter by request method

    let headers = request.headers();
    let header_str = headers_to_key(headers);
    let cache_key = format!("{path}:{header_str}");

    let cached: Option<Vec<u8>> = cache.load(&cache_key).await;
    if let Some(value) = cached {
        if !value.is_empty() {
            let data = Bytes::from(value);
            return Response::new(Body::from(data))
        }
    }

    // Execute the request
    let response = next.run(request).await;
    match response.status().is_success() {
        true => {
            let body = response.into_body();
            let data = axum::body::to_bytes(body, usize::MAX).await.unwrap();
            let data_vec = data.to_vec();
            cache.store(&cache_key, &data_vec).await;
            Response::new(Body::from(data))
        }
        false => response,
    }
}

fn headers_to_key(headers: &HeaderMap) -> String {
    let mut key_parts = Vec::new();

    if let Some(accept) = headers.get("Accept") {
        key_parts.push(format!("Accept:{}", accept.to_str().unwrap_or("")));
    }

    if let Some(auth) = headers.get("Authorization") {
        key_parts.push(format!("Auth:{}", auth.to_str().unwrap_or("")));
    }

    key_parts.join("|")
}
