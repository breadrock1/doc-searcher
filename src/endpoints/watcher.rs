use crate::errors::{ErrorResponse, JsonResponse, Successful, WebError};
use crate::forms::TestExample;
use crate::forms::documents::document::Document;
use crate::forms::documents::preview::DocumentPreview;
use crate::forms::documents::forms::{AnalyseDocsForm, DocumentType, MoveDocsForm};
use crate::services::searcher::service::{UploadedResult, WatcherService};

use actix_multipart::Multipart;
use actix_web::post;
use actix_web::web::block;
use actix_web::web::{Data, Json, Path, Query};
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
            })
        )
    )
)]
#[post("/analysis/fetch")]
async fn fetch_analysis(
    cxt: Context,
    form: Json<AnalyseDocsForm>,
    document_type: Query<DocumentType>,
) -> JsonResponse<Vec<Value>> {
    let client = cxt.get_ref();
    let document_ids = form.0.get_doc_ids();
    let documents = client.analyse_docs(document_ids).await?;
    
    let document_type = document_type.0;
    let values = documents
        .into_iter()
        .map(|doc| document_type.to_value(&doc))
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<Value>>();
    
    Ok(Json(values))
}

#[utoipa::path(
    post,
    path = "/watcher/move/{folder_id}/documents",
    tag = "Documents",
    params(
        (
            "folder_id" = &str,
            description = "Folder id where document is stored",
            example = "test-folder",
        )
    ),
    request_body(
        content = MoveDocsForm,
        example = json!(MoveDocsForm::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful {
                code: 200,
                message: "Done".to_string(),
            })
        ),
        (
            status = 400,
            description = "Failed while moving documents to folder",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while moving documents to folder".to_string(),
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
            })
        )
    )
)]
#[post("/move/{folder_id}/documents")]
async fn move_documents(
    cxt: Context,
    path: Path<String>,
    form: Json<MoveDocsForm>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let folder_id = path.as_ref();
    let move_doc_form = form.0;
    let status = client.move_documents(folder_id, &move_doc_form).await?;
    Ok(Json(status))
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
            })
        )
    )
)]
#[post("/analysis/upload")]
async fn upload_files(
    cxt: Context,
    payload: Multipart,
    document_type: Query<DocumentType>,
) -> JsonResponse<Vec<Value>> {
    let documents = upload_documents(cxt, payload).await?;
    let document_type = document_type.0;
    let values = documents
        .into_iter()
        .map(|doc| document_type.to_value(&doc))
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<Value>>();
    
    Ok(Json(values))
}

async fn upload_documents(cxt: Context, mut payload: Multipart) -> UploadedResult {
    let client = cxt.get_ref();
    let mut collected_docs: Vec<Document> = Vec::default();
    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(|err| WebError::UploadFileError(err.to_string()))?
    {
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap().to_string();
        let filepath = format!("./uploads/{}", filename.as_str());

        let filepath_cln = filepath.clone();
        let create_file_result = block(|| std::fs::File::create(filepath_cln))
            .await
            .unwrap();

        let mut file = create_file_result.unwrap();
        while let Some(read_chunk_result) = field.next().await {
            if read_chunk_result.is_err() {
                let err = read_chunk_result.err().unwrap();
                return Err(WebError::UploadFileError(err.to_string()));
            }

            let data = read_chunk_result.unwrap();
            let file_res = block(move || file.write_all(&data).map(|_| file))
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
        let _ = block(|| std::fs::remove_file(filepath_cln))
            .await
            .unwrap();
    }

    Ok(collected_docs)
}
