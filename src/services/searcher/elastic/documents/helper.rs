use crate::errors::{Successful, WebError, WebErrorEntity, WebResult};
use crate::forms::documents::DocumentsTrait;
use crate::forms::documents::document::Document;
use crate::forms::documents::forms::{DocumentType, MoveDocsForm};
use crate::forms::documents::metadata::DocsArtifacts;
use crate::forms::folders::folder::{ARTIFACTS_FOLDER_ID, HISTORY_FOLDER_ID};
use crate::services::searcher::elastic::helper;
use crate::services::searcher::elastic::context::ElasticContext;
use crate::services::searcher::elastic::documents::store::StoreTrait;
use crate::services::searcher::service::DocumentService;

use elasticsearch::http::response::Response;
use elasticsearch::{BulkParts, CountParts, Elasticsearch, GetParts, IndexParts, UpdateParts};
use elasticsearch::params::Refresh;
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

pub(super) async fn check_duplication(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    folder_id: &str,
    doc_form: &Document,
) -> Result<bool, WebError> {
    let response = elastic
        .count(CountParts::Index(&[folder_id]))
        .body(json!({
            "query" : {
                "term" : {
                    "document_md5" : doc_form.get_doc_id()
                }
            }
        }))
        .send()
        .await
        .map_err(WebError::from)?;

    let result = response.json::<Value>().await.map_or(false, |value| {
        let count = value["count"].as_i64().unwrap_or(0);
        count > 0
    });

    Ok(result)
}

pub(super) async fn extract_document<'de, T: serde::Deserialize<'de>>(response: Response) -> Result<T, WebError> {
    let common_object = response.json::<Value>().await?;
    let document_json = &common_object[&"_source"];
    T::deserialize(document_json.to_owned()).map_err(WebError::from)
}

pub(crate) async fn move_document(
    es_cxt: &ElasticContext,
    doc_id: &str,
    folder_id: &str,
    move_form: &MoveDocsForm,
) -> WebResult<()> {
    let mut document = es_cxt.get_document(folder_id, doc_id).await?;

    let dst_folder = move_form.get_location();
    document.set_folder_id(dst_folder);
    
    let location = std::path::Path::new("./indexer").join(dst_folder);
    let location_str = location.to_str().unwrap_or(dst_folder);
    document.set_folder_path(location_str);

    let doc_artifacts = load_artifacts(es_cxt, ARTIFACTS_FOLDER_ID, dst_folder)
        .await
        .unwrap_or_default();
    document.set_artifacts(&doc_artifacts);
    
    let status = es_cxt.delete_document(folder_id, doc_id).await?;
    if !status.is_success() {
        let msg = status.get_msg().to_string();
        let entity = WebErrorEntity::new(msg);
        return Err(WebError::DeleteDocument(entity))
    }

    let status = es_cxt.create_document(dst_folder, &document, &DocumentType::Document).await?;
    if !status.is_success() {
        let msg = status.get_msg().to_string();
        let entity = WebErrorEntity::new(msg);
        return Err(WebError::CreateDocument(entity));
    }

    let status = update_document(es_cxt, HISTORY_FOLDER_ID, &document).await;
    if status.is_err() {
        let msg = status.err().unwrap();
        log::warn!("failed while removing from history: {}", msg);
    }

    Ok(())
}

pub(crate) async fn update_document(
    es_cxt: &ElasticContext,
    folder_id: &str,
    doc_form: &Document
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

pub(crate) async fn load_artifacts(
    es_cxt: &ElasticContext,
    folder_id: &str,
    doc_id: &str,
) -> WebResult<DocsArtifacts> {
    let elastic = es_cxt.get_cxt().read().await;
    let response = elastic
        .get(GetParts::IndexId(folder_id, doc_id))
        .send()
        .await
        .map_err(WebError::from)?;

    extract_document::<DocsArtifacts>(response).await.map_err(WebError::from)
}
