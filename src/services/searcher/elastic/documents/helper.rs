use crate::errors::{Successful, WebError, WebResult};
use crate::forms::documents::document::Document;
use crate::forms::documents::DocumentsTrait;
use crate::services::searcher::elastic::context::ElasticContext;
use crate::services::searcher::elastic::documents::store::StoreTrait;
use crate::services::searcher::elastic::helper;

use elasticsearch::http::response::Response;
use elasticsearch::params::Refresh;
use elasticsearch::{BulkParts, Elasticsearch, IndexParts, UpdateParts};
use serde_json::{json, Value};
use tokio::sync::RwLockReadGuard;

pub(crate) async fn store_object<T>(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    folder_id: &str,
    doc_form: &T,
) -> WebResult<Successful>
where
    T: DocumentsTrait + StoreTrait<T> + serde::Serialize + Sized,
{
    let response = elastic
        .index(IndexParts::IndexId(folder_id, doc_form.get_doc_id()))
        .refresh(Refresh::True)
        .timeout("1m")
        .body(&doc_form)
        .send()
        .await
        .map_err(WebError::from)?;

    helper::parse_elastic_response(response).await
}

// TODO: Combine those methods to common
pub(crate) async fn store_objects<T>(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    folder_id: &str,
    doc_form: &T,
) -> WebResult<Successful>
where
    T: DocumentsTrait + StoreTrait<T> + serde::Serialize + Sized,
{
    let body = T::create_body(doc_form).await;
    let response = elastic
        .bulk(BulkParts::Index(folder_id))
        .refresh(Refresh::True)
        .timeout("1m")
        .body(body)
        .send()
        .await
        .map_err(WebError::from)?;

    helper::parse_elastic_response(response).await
}

pub(super) async fn extract_document<'de, T: serde::Deserialize<'de>>(
    response: Response,
) -> Result<T, WebError> {
    let common_object = response.json::<Value>().await?;
    let document_json = &common_object[&"_source"];
    T::deserialize(document_json.to_owned()).map_err(WebError::from)
}

pub(crate) async fn update_document(
    es_cxt: &ElasticContext,
    folder_id: &str,
    doc_form: &Document,
) -> WebResult<Successful> {
    let elastic = es_cxt.get_cxt().read().await;
    let doc_id = doc_form.get_doc_id();
    let response = elastic
        .update(UpdateParts::IndexId(folder_id, doc_id))
        .body(&json!({
            "doc": doc_form,
        }))
        .send()
        .await?;

    helper::parse_elastic_response(response).await
}
