use crate::errors::{Successful, WebError, WebErrorEntity, WebResult};
use crate::forms::documents::forms::MoveDocsForm;
use crate::forms::documents::document::Document;
use crate::forms::folders::forms::{CreateFolderForm, DeleteFolderForm};
use crate::services::searcher::elastic::context::ContextOptions;

use reqwest::multipart::Part;
use reqwest::Response;
use serde_derive::Deserialize;
use serde_json::{json, Value};

const MOVE_FILES_URL: &str = "/watcher/files/move";
const ANALYSE_FILES_URL: &str = "/watcher/files/analyse";
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
    folder_form: &CreateFolderForm,
) -> WebResult<Successful> {
    if !folder_form.create_into_watcher() {
        let msg = "Disabled creating directory to watcher service";
        log::warn!("{}: {}", msg, folder_form.get_name());
        return Ok(Successful::success(msg));
    }

    let body = &json!({
        "folder_id": folder_form.get_id(),
        "folder_name": folder_form.get_name()
    });
    let target_url = format!("{}{}", cxt_opts.watcher_address(), CREATE_FOLDER_URL);
    let response = send_watcher_request(target_url.as_str(), body)
        .await
        .map_err(WebError::from)?;

    parse_watcher_response(response).await
}

pub(crate) async fn remove_folder(
    cxt_opts: &ContextOptions,
    folder_id: &str,
    folder_form: &DeleteFolderForm,
) -> WebResult<Successful> {
    if !folder_form.delete_into_watcher() {
        let msg = "Disabled removing directory from watcher service";
        log::warn!("{}: {}", msg, folder_id);
        return Ok(Successful::success(msg));
    }

    let body = &json!({"folder_id": folder_id});
    let target_url = format!("{}{}", cxt_opts.watcher_address(), REMOVE_FOLDER_URL);
    let response = send_watcher_request(target_url.as_str(), body)
        .await
        .map_err(WebError::from)?;

    parse_watcher_response(response).await
}

pub(crate) async fn launch_analysis(
    cxt_opts: &ContextOptions,
    doc_ids: &[String]
) -> WebResult<Vec<Document>> {
    let body = &json!({"document_ids": doc_ids});
    let target_url = format!("{}{}", cxt_opts.watcher_address(), ANALYSE_FILES_URL);
    let response = send_watcher_request(target_url.as_str(), body)
        .await
        .map_err(WebError::from)?;

    response
        .json::<Vec<Document>>()
        .await
        .map_err(WebError::from)
}

pub(crate) async fn move_docs_to_folder(
    cxt_opts: &ContextOptions,
    src_folder_id: &str, 
    move_form: &MoveDocsForm,
) -> WebResult<Successful> {
    if !move_form.use_watcher() {
        let msg = "Disabled moving documents into watcher";
        log::warn!("{}: {} -> {}", msg, src_folder_id, move_form.get_location());
        return Ok(Successful::success(msg));
    }

    let dst_folder_id = move_form.get_location();
    let document_ids = move_form.get_doc_ids();

    let body = &json!({
        "document_ids": document_ids,
        "location": dst_folder_id,
        "src_folder_id": src_folder_id,
    });
    let target_url = format!("{}{}", cxt_opts.watcher_address(), MOVE_FILES_URL);
    let response = send_watcher_request(target_url.as_str(), body)
        .await
        .map_err(WebError::from)?;

    response.json::<Successful>().await.map_err(WebError::from)
}

pub async fn parse_watcher_response(response: Response) -> WebResult<Successful> {
    if !response.status().is_success() {
        return Err(extract_exception(response).await);
    }

    Ok(Successful::success("Ok"))
}

async fn extract_exception(response: Response) -> WebError {
    let parse_result = response
        .json::<ResponseError>()
        .await
        .map_err(WebError::from);

    match parse_result {
        Err(err) => err,
        Ok(err_resp) => {
            let entity = WebErrorEntity::new(err_resp.message);
            WebError::UnknownError(entity)
        },
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
) -> Result<Vec<Document>, WebError> {
    let content: Vec<u8> = tokio::fs::read(filepath).await?;
    let part = Part::bytes(content).file_name(filename);
    let form = reqwest::multipart::Form::new().part("files", part);

    let target_url = format!("{}{}", cxt_opts.watcher_address(), UPLOAD_FILES_URL);
    let response = reqwest::Client::new()
        .post(target_url)
        .multipart(form)
        .send()
        .await
        .map_err(WebError::from)?;

    response
        .json::<Vec<Document>>()
        .await
        .map_err(WebError::from)
}
