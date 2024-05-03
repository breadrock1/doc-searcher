use crate::errors::{JsonResponse, SuccessfulResponse, WebError};
use crate::services::elastic::context::ContextOptions;

use wrappers::document::DocumentPreview;

use actix_web::{HttpResponse, ResponseError, web};
use serde_json::{json, Value};
use wrappers::search_params::SearchParams;

const MOVE_FILES_URL: &str = "/files/move";
const ANALYSE_FILES_URL: &str = "/files/analyse";
const UNRECOGNIZED_FILES_URL: &str = "/files/unrecognized";

pub async fn launch_docs_analysis(
    cxt_opts: &ContextOptions,
    document_ids: &[String],
) -> Result<Vec<DocumentPreview>, WebError> {
    let body = &json!({"document_ids": document_ids});
    let target_url = format!("{}/{}", cxt_opts.watcher_service_host(), ANALYSE_FILES_URL);
    match send_watcher_request(target_url.as_str(), body).await {
        Err(err) => Err(WebError::ResponseError(err.to_string())),
        Ok(response) => Ok(
            response
                .json::<Vec<DocumentPreview>>()
                .await
                .unwrap()
        ),
    }
}

pub async fn move_docs_to_folder(
    cxt_opts: &ContextOptions,
    folder_id: &str, 
    document_ids: &[String]
) -> HttpResponse {
    let body = &json!({"folder_id": folder_id, "document_ids": document_ids});
    let target_url = format!("{}/{}", cxt_opts.watcher_service_host(), MOVE_FILES_URL);
    match send_watcher_request(target_url.as_str(), body).await {
        Err(err) => WebError::ResponseError(err.to_string()).error_response(),
        Ok(response) => {
            response
                .json::<SuccessfulResponse>()
                .await
                .unwrap()
                .to_response()
        },
    }
}

pub async fn get_unrecognized_documents(
    cxt_opts: &ContextOptions,
    _s_params: &SearchParams,
) -> JsonResponse<Vec<DocumentPreview>> {
    // TODO: Implement response result filtering by fields.
    let target_url = format!("{}/{}", cxt_opts.watcher_service_host(), UNRECOGNIZED_FILES_URL);
    let client = reqwest::Client::new();
    let response = client
        .get(target_url)
        .send()
        .await
        .map_err(|err| WebError::ResponseError(err.to_string()))?;
    
    Ok(web::Json(
        response
            .json::<Vec<DocumentPreview>>()
            .await
            .map_err(|err| WebError::ResponseError(err.to_string()))?
    ))
}

async fn send_watcher_request(
    target_url: &str,
    json_body: &Value
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .post(target_url)
        .json(json_body)
        .send()
        .await
}
