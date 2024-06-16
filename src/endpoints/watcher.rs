use std::error::Error;
use crate::errors::{ErrorResponse, JsonResponse, WebError, WebErrorEntity};
use crate::forms::TestExample;
use crate::forms::documents::document::Document;
use crate::forms::documents::preview::DocumentPreview;
use crate::forms::documents::forms::{AnalyseDocsForm, DocTypeQuery};
use crate::services::searcher::elastic::helper;
use crate::services::searcher::service::{UploadedResult, WatcherService};

use actix_multipart::Multipart;
use actix_web::post;
use actix_web::web::block;
use actix_web::web::{Data, Json, Query};
use futures::{StreamExt, TryStreamExt};
use serde_json::Value;
use std::io::Write;

type Context = Data<Box<dyn WatcherService>>;

#[utoipa::path(
    post,
    path = "/watcher/analysis/fetch",
    tag = "Watcher",
    params(
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document"
        )
    ),
    request_body(
        content = AnalyseDocsForm,
        example = json!(AnalyseDocsForm::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
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
                attachments: None,
            })
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 503,
                error: "Server error".to_string(),
                message: "Server does not available".to_string(),
                attachments: None,
            })
        )
    )
)]
#[post("/analysis/fetch")]
async fn fetch_analysis(
    cxt: Context,
    form: Json<AnalyseDocsForm>,
    document_type: Query<DocTypeQuery>,
) -> JsonResponse<Vec<Value>> {
    let client = cxt.get_ref();
    let document_ids = form.0.get_doc_ids();
    let doc_type = document_type.0.get_type();
    let documents = client.analyse_docs(document_ids, &doc_type).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    post,
    path = "/watcher/analysis/upload",
    tag = "Watcher",
    params(
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document"
        )
    ),
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
                attachments: None,
            })
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 503,
                error: "Server error".to_string(),
                message: "Server does not available".to_string(),
                attachments: None,
            })
        )
    )
)]
#[post("/analysis/upload")]
async fn upload_files(
    cxt: Context,
    payload: Multipart,
    document_type: Query<DocTypeQuery>,
) -> JsonResponse<Vec<Value>> {
    let documents = upload_documents(cxt, payload).await?;
    let document_type = document_type.0.get_type();
    let values = helper::to_unified_docs(documents, &document_type);
    Ok(Json(values))
}

async fn upload_documents(cxt: Context, mut payload: Multipart) -> UploadedResult {
    let client = cxt.get_ref();
    let mut collected_docs: Vec<Document> = Vec::default();
    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(convert_err)?
    {
        let content_disposition = field.content_disposition();
        let filename = content_disposition
            .get_filename()
            .ok_or_else(|| {
                let msg = "failed while get filename".to_string();
                let entity = WebErrorEntity::new(msg);
                WebError::UploadFileError(entity)
            })?
            .to_string();

        let filepath = format!("./uploads/{}", filename);
        let filepath_cln = filepath.clone();
        let create_file_result = block(|| std::fs::File::create(filepath_cln))
            .await
            .map_err(|err| {
                let entity = WebErrorEntity::new(err.to_string());
                WebError::UploadFileError(entity)
            })?;

        let mut file = create_file_result.map_err(convert_err)?;
        while let Some(read_chunk_result) = field.next().await {
            let data = read_chunk_result.map_err(convert_err)?;
            let file_res = block(move || file.write_all(&data).map(|_| file))
                .await
                .map_err(|err| {
                    let entity = WebErrorEntity::new(err.to_string());
                    WebError::UploadFileError(entity)
                })?;

            file = file_res.unwrap()
        }

        let documents = client
            .upload_files(filename.as_str(), filepath.as_str())
            .await
            .map_err(convert_err)?;

        collected_docs.extend_from_slice(documents.as_slice());

        let filepath_cln = filepath.clone();
        let _ = block(|| std::fs::remove_file(filepath_cln))
            .await
            .map_err(|err| {
                let entity = WebErrorEntity::new(err.to_string());
                WebError::UploadFileError(entity)
            })?;
    }

    Ok(collected_docs)
}

fn convert_err<T>(err: T) -> WebError
where
    T: Error
{
    let entity = WebErrorEntity::new(err.to_string());
    WebError::UploadFileError(entity)
}
