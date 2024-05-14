use crate::errors::WebError;

use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::response::Response;
use elasticsearch::http::Method;
use elasticsearch::Elasticsearch;
use serde_json::Value;

pub(crate) async fn get_all_clusters(elastic: &Elasticsearch) -> Result<Response, WebError> {
    elastic
        .send(
            Method::Get,
            "/_cat/nodes",
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(b"".as_ref()),
            None,
        )
        .await
        .map_err(WebError::from)
}
