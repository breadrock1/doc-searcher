use crate::errors::{SuccessfulResponse, WebError, WebResult};
use crate::forms::documents::forms::MoveDocumentsForm;
use crate::forms::folder::FolderForm;
use crate::forms::preview::DocumentPreview;
use crate::forms::s_params::SearchParams;
use crate::services::elastic::context::ContextOptions;

use reqwest::multipart::Part;
use reqwest::Response;
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
    #[allow(dead_code)]
    status: u16,
    message: String,
}

pub(crate) async fn create_folder(
    cxt_opts: &ContextOptions,
    folder_form: &FolderForm,
) -> WebResult {
    let body = &json!({
        "folder_id": folder_form.get_id(),
        "folder_name": folder_form.get_name()
    });
    let host = cxt_opts.watcher_service_host();
    let target_url = format!("{}{}", host, CREATE_FOLDER_URL);
    let response = send_watcher_request(target_url.as_str(), body)
        .await
        .map_err(WebError::from)?;

    parse_watcher_response(response).await
}

pub(crate) async fn remove_folder(cxt_opts: &ContextOptions, folder_id: &str) -> WebResult {
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
    move_form: &MoveDocumentsForm,
) -> WebResult {
    let dst_folder_id = move_form.get_folder_id();
    let src_folder_id = move_form.get_src_folder_id();
    let document_ids = move_form.get_document_ids();

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

pub async fn parse_watcher_response(response: Response) -> WebResult {
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
        Ok(err_resp) => WebError::UnknownError(err_resp.message),
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

    response
        .json::<Vec<DocumentPreview>>()
        .await
        .map_err(WebError::from)
}
