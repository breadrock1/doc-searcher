use crate::endpoints::SearcherData;
use crate::errors::{ErrorResponse, JsonResponse, SuccessfulResponse, WebError};

use wrappers::document::DocumentPreview;
use wrappers::file_form::LoadFileForm;
use wrappers::TestExample;

use actix_multipart::Multipart;
use actix_web::{post, web};
use actix_web::{HttpRequest, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use reqwest::multipart::Part;
use std::io::Write;

#[utoipa::path(
    post,
    path = "/file/load",
    tag = "Files",
    request_body(
        content = LoadFileForm,
        example = json!({
            "file_path": "/tmp/test.txt",
            "folder_id": "test_folder",
        }),
    ),
    responses(
        (
            status = 200,
            description = "Successful", 
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
                code: 200,
                message: "Done".to_string(),
            })
        ),
        (
            status = 400,
            description = "Failed while loading files", 
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while loading files".to_string(),
            })
        ),
    )
)]
#[post("/load")]
async fn load_file(cxt: SearcherData, form: web::Json<LoadFileForm>) -> HttpResponse {
    let client = cxt.get_ref();
    let file_path = form.get_path();
    let bucket_name = form.get_bucket();
    client.load_file_to_bucket(bucket_name, file_path).await
}

#[utoipa::path(
    post,
    path = "/file/download",
    tag = "Files",
    request_body(
        content = LoadFileForm,
        example = json!({
            "file_path": "/tmp/test.txt",
            "folder_id": "test_folder",
        }),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
                code: 200,
                message: "Done".to_string(),
            })
        ),
        (
            status = 400,
            description = "Failed while downloading files", 
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while downloading files".to_string(),
            })
        ),
    )
)]
#[post("/download")]
async fn download_file(
    cxt: SearcherData,
    req: HttpRequest,
    form: web::Json<LoadFileForm>,
) -> HttpResponse {
    let client = cxt.get_ref();
    let file_path = form.get_path();
    let bucket_name = form.get_bucket();
    client
        .download_file(bucket_name, file_path)
        .await
        .unwrap()
        .into_response(&req)
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
async fn upload_files(cxt: SearcherData, payload: Multipart) -> JsonResponse<Vec<DocumentPreview>> {
    let _client = cxt.get_ref();
    match extract_multipart(payload).await {
        Err(err) => Err(WebError::ResponseError(err.to_string())),
        Ok(result) => Ok(web::Json(result)),
    }
}

async fn extract_multipart(mut payload: Multipart) -> Result<Vec<DocumentPreview>, anyhow::Error> {
    let mut collected_docs: Vec<DocumentPreview> = Vec::default();
    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(|err| extract_error(err, "Failed extract payload"))?
    {
        let content_type = field.content_disposition();
        let filename = content_type.get_filename().unwrap().to_string();
        let filepath = format!("./uploads/{}", filename.as_str());

        let filepath_cln = filepath.clone();
        let create_file_result = web::block(|| std::fs::File::create(filepath_cln))
            .await
            .unwrap();

        let mut file = create_file_result.unwrap();
        while let Some(read_chunk_result) = field.next().await {
            if read_chunk_result.is_err() {
                let err = read_chunk_result.err().unwrap();
                return Err(extract_error(err, "extracting chunk"));
            }

            let data = read_chunk_result.unwrap();
            let file_res = web::block(move || file.write_all(&data).map(|_| file))
                .await
                .unwrap();

            file = file_res.unwrap()
        }

        let content: Vec<u8> = tokio::fs::read(filepath.as_str()).await?;
        let part = Part::bytes(content).file_name(filename);
        let form = reqwest::multipart::Form::new().part("files", part);

        let client = reqwest::Client::new();
        let response_result = client
            .post("http://localhost:2893/watcher/files/upload")
            .multipart(form)
            .send()
            .await;

        match response_result {
            Err(err) => log::error!("{}", err.to_string()),
            Ok(response) => {
                let previews_result = response.json::<Vec<DocumentPreview>>().await;

                if previews_result.is_err() {
                    let err = previews_result.err().unwrap();
                    log::error!("Failed while deserialize response: {}", err);
                    continue;
                }
                collected_docs.extend_from_slice(previews_result.unwrap().as_slice());
            }
        }

        let filepath_cln_2 = filepath.clone();
        let _ = web::block(|| std::fs::remove_file(filepath_cln_2)).await?;
    }

    Ok(collected_docs)
}

fn extract_error<T: std::fmt::Debug + std::fmt::Display>(err: T, msg: &str) -> anyhow::Error {
    let msg = format!("Failed while {}: {}", msg, err);
    log::error!("{}", msg);
    anyhow::Error::msg(msg)
}

#[cfg(test)]
mod loader_endpoints {
    use crate::services::own_engine::context::OtherContext;
    use crate::services::SearcherService;

    use actix_web::test;

    #[test]
    async fn test_load_file() {
        let file_path = "src/crates/loader/resources/test.txt";
        let other_context = OtherContext::new("test".to_string());
        let response = other_context
            .load_file_to_bucket("test_folder", file_path)
            .await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }
}
