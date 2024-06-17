use crate::errors::{Successful, WebError, WebErrorEntity, WebResult};
use crate::forms::documents::DocumentsTrait;
use crate::forms::documents::document::Document;
use crate::forms::documents::forms::{DocumentType, MoveDocsForm};
use crate::services::searcher::elastic::helper;
use crate::services::searcher::elastic::context::ElasticContext;
use crate::services::searcher::elastic::documents::store::StoreTrait;
use crate::services::searcher::service::DocumentService;

use elasticsearch::http::response::Response;
use elasticsearch::{BulkParts, CountParts, Elasticsearch};
use elasticsearch::http::Method;
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::sync::RwLockReadGuard;
use crate::forms::folders::folder::HISTORY_FOLDER_ID;
use crate::services::searcher::elastic::documents::update::UpdateTrait;

pub(crate) async fn store_object<T>(
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

pub(super) async fn extract_document(response: Response) -> Result<Document, WebError> {
    let common_object = response.json::<Value>().await?;
    let document_json = &common_object[&"_source"];
    Document::deserialize(document_json).map_err(WebError::from)
}

pub(crate) async fn move_document(
    es_cxt: &ElasticContext,
    doc_id: &str,
    folder_id: &str,
    move_form: &MoveDocsForm,
) -> WebResult<()> {
    let dst_folder = move_form.get_location();

    let mut document = es_cxt.get_document(folder_id, doc_id).await?;
    let status = es_cxt.delete_document(folder_id, doc_id).await?;
    if !status.is_success() {
        let msg = status.get_msg().to_string();
        let entity = WebErrorEntity::new(msg);
        return Err(WebError::DeleteDocument(entity))
    }

    document.set_folder_id(dst_folder);
    let status = es_cxt.create_document(dst_folder, &document, &DocumentType::Document).await?;
    if !status.is_success() {
        let msg = status.get_msg().to_string();
        let entity = WebErrorEntity::new(msg);
        return Err(WebError::CreateDocument(entity));
    }

    let status = Document::update(es_cxt, HISTORY_FOLDER_ID, &document).await;
    if !status.is_err() {
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
    let s_path = format!("/{}/_update/{}", folder_id, doc_form.get_doc_id());
    
    let mut bytes: Vec<u8> = Vec::new();
    serde_json::to_writer(&mut bytes, doc_form).unwrap();
    let response = helper::send_elrequest(
        &elastic,
        Method::Put,
        Some(bytes.as_slice()),
        s_path.as_str(),
    )
    .await?;
    
    helper::parse_elastic_response(response).await
}
