use crate::errors::{SuccessfulResponse, WebError};
use crate::services::elastic::context::ContextOptions;

use wrappers::document::DocumentPreview;
use wrappers::s_params::SearchParams;

use reqwest::Response;
use reqwest::multipart::Part;
use serde_derive::Deserialize;
use serde_json::{json, Value};

const MOVE_FILES_URL: &str = "/watcher/files/move";
const ANALYSE_FILES_URL: &str = "/watcher/files/analyse";
const UNRECOGNIZED_FILES_URL: &str = "/watcher/files/unrecognized";
const CREATE_FOLDER_URL: &str = "/watcher/folders/create";
const REMOVE_FOLDER_URL: &str = "/watcher/folders/remove";
const UPLOAD_FILES_URL: &str = "/watcher/files/upload";

#[derive(Deserialize)]
struct ResponseError {
    status: u16,
    message: String,
}

pub(crate) async fn create_folder(
    cxt_opts: &ContextOptions,
    folder_id: &str,
) -> Result<SuccessfulResponse, WebError> {
    let body = &json!({"folder_id": folder_id});
    let host = cxt_opts.watcher_service_host();
    let target_url = format!("{}{}", host, CREATE_FOLDER_URL);
    let response = send_watcher_request(target_url.as_str(), body)
        .await
        .map_err(WebError::from)?;

    parse_watcher_response(response).await
}

pub(crate) async fn remove_folder(
    cxt_opts: &ContextOptions,
    folder_id: &str,
) -> Result<SuccessfulResponse, WebError> {
    let body = &json!({"folder_id": folder_id});
    let host = cxt_opts.watcher_service_host();
    let target_url = format!("{}{}", host, REMOVE_FOLDER_URL);
    let response = send_watcher_request(target_url.as_str(), body)
        .await
        .map_err(WebError::from)?;

    parse_watcher_response(response).await
}

pub(crate) async fn launch_analysis(
    cxt_opts: &ContextOptions,
    document_ids: &[String],
) -> Result<Vec<DocumentPreview>, WebError> {
    let body = &json!({"document_ids": document_ids});
    let host = cxt_opts.watcher_service_host();
    let target_url = format!("{}{}", host, ANALYSE_FILES_URL);
    let response = send_watcher_request(target_url.as_str(), body)
        .await
        .map_err(WebError::from)?;

    response
        .json::<Vec<DocumentPreview>>()
        .await
        .map_err(WebError::from)
}

pub(crate) async fn move_docs_to_folder(
    cxt_opts: &ContextOptions,
    dst_folder_id: &str,
    src_folder_id: &str,
    document_ids: &[String],
) -> Result<SuccessfulResponse, WebError> {
    let body = &json!({
        "document_ids": document_ids,
        "location": dst_folder_id,
        "src_folder_id": src_folder_id,
    });
    let host = cxt_opts.watcher_service_host();
    let target_url = format!("{}{}", host, MOVE_FILES_URL);
    let response = send_watcher_request(target_url.as_str(), body)
        .await
        .map_err(WebError::from)?;

    response
        .json::<SuccessfulResponse>()
        .await
        .map_err(WebError::from)
}

pub(crate) async fn get_unrecognized_docs(
    cxt_opts: &ContextOptions,
    _s_params: &SearchParams,
) -> Result<Vec<DocumentPreview>, WebError> {
    // TODO: Implement response result filtering by fields.
    let host = cxt_opts.watcher_service_host();
    let target_url = format!("{}{}", host, UNRECOGNIZED_FILES_URL);
    let response = reqwest::Client::new()
        .get(target_url)
        .send()
        .await
        .map_err(WebError::from)?;

    response
        .json::<Vec<DocumentPreview>>()
        .await
        .map_err(WebError::from)
}

pub async fn parse_watcher_response(response: Response) -> Result<SuccessfulResponse, WebError> {
    if !response.status().is_success() {
        return Err(extract_exception(response).await);
    }

    Ok(SuccessfulResponse::success("Ok"))
}

async fn extract_exception(response: Response) -> WebError {
    let parse_result = response
        .json::<ResponseError>()
        .await
        .map_err(WebError::from);

    match parse_result {
        Err(err) => err,
        Ok(err_resp) => {
            WebError::UnknownError(err_resp.message)
        }
    }
}

async fn send_watcher_request(
    target_url: &str,
    json_body: &Value,
) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client.post(target_url).json(json_body).send().await
}

pub(crate) async fn translate_multipart_form(
    cxt_opts: &ContextOptions,
    filename: String,
    filepath: String,
) -> Result<Vec<DocumentPreview>, WebError> {
    let content: Vec<u8> = tokio::fs::read(filepath).await?;
    let part = Part::bytes(content).file_name(filename);
    let form = reqwest::multipart::Form::new().part("files", part);

    let watcher_host = cxt_opts.watcher_service_host();
    let target_url = format!("{}{}", watcher_host, UPLOAD_FILES_URL);
    let response = reqwest::Client::new()
        .post(target_url)
        .multipart(form)
        .send()
        .await
        .map_err(WebError::from)?;

    response.json::<Vec<DocumentPreview>>()
        .await
        .map_err(WebError::from)
}
