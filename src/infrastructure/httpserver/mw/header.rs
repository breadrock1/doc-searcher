use axum::extract::{Request, State};
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::Response;
use gset::Getset;
use std::sync::Arc;
use tower_http::add_extension::AddExtensionLayer;

use crate::application::structures::{UserInfo, UserInfoBuilder};

const HEADER_FIELDS: [&str; 1] = ["X-Sova-User-Id"];

pub struct HeadersExtractor;

#[derive(Debug, Clone, Getset)]
pub struct UserInfoHeader {
    #[getset(get, vis = "pub")]
    user_id: String,
}

impl From<&UserInfoHeader> for UserInfo {
    fn from(value: &UserInfoHeader) -> Self {
        UserInfoBuilder::default()
            .user_id(value.user_id.clone())
            .build()
            .unwrap()
    }
}

pub async fn enable_header_extractor_mw(app: axum::Router) -> anyhow::Result<axum::Router> {
    let extractor_state = HeadersExtractor {};
    let state_arc = Arc::new(extractor_state);

    let ext_layer = AddExtensionLayer::new(state_arc.clone());
    let mw_layer = axum::middleware::from_fn_with_state(state_arc, extract_headers);
    let tower_layer = tower::ServiceBuilder::new()
        .layer(ext_layer)
        .layer(mw_layer);

    Ok(app.layer(tower_layer))
}

async fn extract_headers(
    State(_): State<Arc<HeadersExtractor>>,
    mut request: Request,
    next: Next,
) -> Response {
    let user_info = headers_to_key(request.headers());
    request.extensions_mut().insert(user_info);
    next.run(request).await
}

fn headers_to_key(headers: &HeaderMap) -> Option<UserInfoHeader> {
    match headers.get(HEADER_FIELDS[0]) {
        None => None,
        Some(value) => {
            let value_str = value.to_str().ok();
            value_str.map(|it| UserInfoHeader {
                user_id: it.to_string(),
            })
        }
    }
}
