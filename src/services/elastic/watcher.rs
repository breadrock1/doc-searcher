use crate::errors::{SuccessfulResponse, WebError};
use crate::services::elastic::context::ContextOptions;

use wrappers::document::DocumentPreview;
use wrappers::search_params::SearchParams;

use serde_json::{json, Value};

const MOVE_FILES_URL: &str = "/watcher/files/move";
const ANALYSE_FILES_URL: &str = "/watcher/files/analyse";
const UNRECOGNIZED_FILES_URL: &str = "/watcher/files/unrecognized";
const CREATE_FOLDER_URL: &str = "/watcher/folders/create";

pub(crate) async fn create_watcher_folder(
    cxt_opts: &ContextOptions,
    folder_id: &str,
) -> Result<SuccessfulResponse, WebError> {
    let body = &json!({"folder_name": folder_id});
    let target_url = format!("{}{}", cxt_opts.watcher_service_host(), CREATE_FOLDER_URL);
    match send_watcher_request(target_url.as_str(), body).await {
        Err(err) => Err(WebError::ResponseError(err.to_string())),
        Ok(response) => {
            response
                .json::<SuccessfulResponse>()
                .await
                .map_err(|err| WebError::ResponseError(err.to_string()))
        },
    }
}

pub async fn launch_docs_analysis(
    cxt_opts: &ContextOptions,
    document_ids: &[String],
) -> Result<Vec<DocumentPreview>, WebError> {
    let body = &json!({"document_ids": document_ids});
    let target_url = format!("{}{}", cxt_opts.watcher_service_host(), ANALYSE_FILES_URL);
    match send_watcher_request(target_url.as_str(), body).await {
        Err(err) => Err(WebError::ResponseError(err.to_string())),
        Ok(response) => {
            let status = &response.status();
            println!("{}", status.as_str());
            if status.as_u16() == 102 {
                return Err(WebError::ResponseContinues("Processing".to_string()))
            }

            response
                .json::<Vec<DocumentPreview>>()
                .await
                .map_err(|err| WebError::ResponseError(err.to_string()))
        }
    }
}

pub(crate) async fn move_docs_to_folder(
    cxt_opts: &ContextOptions,
    folder_id: &str,
    document_ids: &[String],
) -> Result<SuccessfulResponse, WebError> {
    let body = &json!({"folder_id": folder_id, "document_ids": document_ids});
    let target_url = format!("{}{}", cxt_opts.watcher_service_host(), MOVE_FILES_URL);
    match send_watcher_request(target_url.as_str(), body).await {
        Err(err) => Err(WebError::ResponseError(err.to_string())),
        Ok(response) => response
            .json::<SuccessfulResponse>()
            .await
            .map_err(|err| WebError::ResponseError(err.to_string())),
    }
}

pub async fn get_unrecognized_documents(
    cxt_opts: &ContextOptions,
    _s_params: &SearchParams,
) -> Result<Vec<DocumentPreview>, WebError> {
    // TODO: Implement response result filtering by fields.
    let target_url = format!(
        "{}{}",
        cxt_opts.watcher_service_host(),
        UNRECOGNIZED_FILES_URL
    );
    let response = reqwest::Client::new()
        .get(target_url)
        .send()
        .await
        .map_err(|err| WebError::ResponseError(err.to_string()))?;

    response
        .json::<Vec<DocumentPreview>>()
        .await
        .map_err(|err| WebError::ResponseError(err.to_string()))
}

async fn send_watcher_request(
    target_url: &str,
    json_body: &Value,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client.post(target_url).json(json_body).send().await
}
