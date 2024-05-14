use crate::errors::{ErrorResponse, JsonResponse, WebError};
use crate::forms::document::{AnalyseDocumentsForm, DocumentPreview};
use crate::forms::TestExample;
use crate::services::searcher::{UploadedResult, WatcherService};

use actix_multipart::Multipart;
use actix_web::{post, web};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

type Context = web::Data<Box<dyn WatcherService>>;

#[utoipa::path(
    post,
    path = "/watcher/analyse",
    tag = "Watcher",
    request_body(
        content = AnalyseDocumentsForm,
        example = json!(AnalyseDocumentsForm::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = SuccessfulResponse,
            example = json!(vec![DocumentPreview::test_example(None)])
        ),
            (
            status = 400,
            description = "Failed while analysing documents",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while analysing documents".to_string(),
            })
        ),
    )
)]
#[post("/analyse")]
async fn analyse_documents(
    cxt: Context,
    form: web::Json<AnalyseDocumentsForm>,
) -> JsonResponse<Vec<DocumentPreview>> {
    let client = cxt.get_ref();
    let document_ids = form.0.document_ids;
    client.launch_analysis(document_ids.as_slice()).await
}

#[utoipa::path(
    post,
    path = "/watcher/upload",
    tag = "Watcher",
    request_body(
        content_type = "multipart/formdata",
        content = Multipart,
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Vec<DocumentPreview>,
            example = json!(vec![DocumentPreview::test_example(None)]),
        ),
        (
            status = 400,
            description = "Failed while downloading files",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while uploading files to watcher".to_string(),
            })
        ),
    )
)]
#[post("/upload")]
async fn upload_files(cxt: Context, payload: Multipart) -> JsonResponse<Vec<DocumentPreview>> {
    match upload_documents(cxt, payload).await {
        Err(err) => Err(WebError::UnknownError(err.to_string())),
        Ok(result) => Ok(web::Json(result)),
    }
}

async fn upload_documents(cxt: Context, mut payload: Multipart) -> UploadedResult {
    let client = cxt.get_ref();
    let mut collected_docs: Vec<DocumentPreview> = Vec::default();
    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(|err| WebError::UploadFileError(err.to_string()))?
    {
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap().to_string();
        let filepath = format!("./uploads/{}", filename.as_str());

        let filepath_cln = filepath.clone();
        let create_file_result = web::block(|| std::fs::File::create(filepath_cln))
            .await
            .unwrap();

        let mut file = create_file_result.unwrap();
        while let Some(read_chunk_result) = field.next().await {
            if read_chunk_result.is_err() {
                let err = read_chunk_result.err().unwrap();
                return Err(WebError::UploadFileError(err.to_string()));
            }

            let data = read_chunk_result.unwrap();
            let file_res = web::block(move || file.write_all(&data).map(|_| file))
                .await
                .unwrap();

            file = file_res.unwrap()
        }

        let upload_result = client
            .upload_files(filename.as_str(), filepath.as_str())
            .await;

        if upload_result.is_err() {
            let err = upload_result.err().unwrap();
            log::error!("Failed while deserialize response: {}", err);
            continue;
        }

        let documents = upload_result.unwrap();
        collected_docs.extend_from_slice(documents.as_slice());

        let filepath_cln = filepath.clone();
        let _ = web::block(|| std::fs::remove_file(filepath_cln))
            .await
            .unwrap();
    }

    Ok(collected_docs)
}
