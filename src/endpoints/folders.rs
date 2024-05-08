use crate::endpoints::SearcherData;
use crate::errors::{ErrorResponse, SuccessfulResponse, WebError};
use crate::errors::{JsonResponse, PaginateJsonResponse};

use actix_web::{delete, get, post, web, HttpResponse, ResponseError};

use wrappers::bucket::{Folder, FolderForm};
use wrappers::document::DocumentPreview;
use wrappers::scroll::PaginatedResult;
use wrappers::search_params::SearchParams;
use wrappers::TestExample;

#[utoipa::path(
    get,
    path = "/folders/",
    tag = "Folders",
    responses(
        (
            status = 200,
            description = "Successful",
            body = [Folder],
            example = json!(vec![Folder::default()]),
        ),
        (
            status = 400,
            description = "Failed while getting all folders",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting folders".to_string(),
            }),
        ),
    )
)]
#[get("/")]
async fn all_folders(cxt: SearcherData) -> JsonResponse<Vec<Folder>> {
    let client = cxt.get_ref();
    client.get_all_folders().await
}

#[utoipa::path(
    get,
    path = "/folders/{folder_id}",
    tag = "Folders",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get details",
            example = "test_folder",
        )
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Folder,
            example = json!(Folder::default())
        ),
        (
            status = 400,
            description = "Failed while getting folder by id",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting folder by id".to_string(),
            })
        ),
    )
)]
#[get("/{folder_id}")]
async fn get_folder(cxt: SearcherData, path: web::Path<String>) -> JsonResponse<Folder> {
    let client = cxt.get_ref();
    client.get_folder(path.as_str()).await
}

#[utoipa::path(
    post,
    path = "/folders/create",
    tag = "Folders",
    request_body(
        content = FolderForm,
        example = json!({
            "folder_id": "test_folder"
        })
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
                code: 200,
                message: "Done".to_string(),
            }),
        ),
        (
            status = 400,
            description = "Failed while creating new folder",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while creating new folder".to_string(),
            }),
        ),
    )
)]
#[post("/create")]
async fn create_folder(cxt: SearcherData, form: web::Json<FolderForm>) -> HttpResponse {
    let client = cxt.get_ref();
    let folder_form = form.0;
    client.create_folder(&folder_form).await
}

#[utoipa::path(
    delete,
    path = "/folders/{folder_id}",
    tag = "Folders",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to delete",
            example = "test_folder",
        )
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
                code: 200,
                message: "Done".to_string(),
            }),
        ),
        (
            status = 400,
            description = "Failed while deleting folder",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while deleting folder".to_string(),
            }),
        ),
    )
)]
#[delete("/{folder_id}")]
async fn delete_folder(cxt: SearcherData, path: web::Path<String>) -> HttpResponse {
    let client = cxt.get_ref();
    let folder_id = path.to_string();
    client.delete_folder(folder_id.as_str()).await
}

#[utoipa::path(
    post,
    path = "/folders/global",
    tag = "Folders",
    responses(
        (
            status = 200,
            description = "Successful",
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
                code: 200,
                message: "Done".to_string(),
            }),
        ),
        (
            status = 400,
            description = "Failed while creating global folders",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while creating global folders".to_string(),
            }),
        ),
    )
)]
#[post("/global")]
async fn create_global_folders(cxt: SearcherData) -> HttpResponse {
    let client = cxt.get_ref();
    let mut collected_errs = Vec::default();
    for global_folders_id in ["history", "unrecognized"] {
        let folder_form = FolderForm::new(global_folders_id);
        let response = client.create_folder(&folder_form).await;
        if response.status() != 200 {
            collected_errs.push(global_folders_id);
        }
    }

    if !collected_errs.is_empty() {
        let folders_str = collected_errs.join(", ");
        let msg = format!("Failed while creating global buckets: {}", folders_str);
        return WebError::CreateBucket(msg).error_response();
    }

    SuccessfulResponse::ok_response("Done")
}

#[utoipa::path(
    post,
    path = "/folders/{folder_id}/documents",
    tag = "Folders",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get stored documents",
            example = "test_folder",
        ),
    ),
    request_body(
        content = SearchParams,
        example = json!(SearchParams::test_example(Some("Ocean Carrier"))),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = PaginatedResult<Vec<Document>>,
            example = json!(PaginatedResult::<Vec<DocumentPreview>>::new_with_id(
                vec![DocumentPreview::test_example(None)],
                "DXF1ZXJ5QW5kRmV0Y2gBAD4WYm9lafytZndUQlNsdDcwakFMNjU1QQ==".to_string(),
            )),
        ),
        (
            status = 400,
            description = "Failed while getting stored documents into folder",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting stored documents into folder".to_string(),
            }),
        ),
    )
)]
#[post("/{folder_id}/documents")]
async fn get_folder_documents(
    cxt: SearcherData,
    path: web::Path<String>,
    form: web::Json<SearchParams>,
) -> PaginateJsonResponse<Vec<DocumentPreview>> {
    let client = cxt.get_ref();
    let search_form = form.0;
    match client
        .get_folder_documents(path.as_str(), Some(search_form))
        .await
    {
        Err(err) => Err(err),
        Ok(value) => {
            let scroll_id = value.get_scroll_id().cloned();
            let preview = value
                .get_founded()
                .to_owned()
                .into_iter()
                .map(DocumentPreview::from)
                .collect();

            Ok(web::Json(PaginatedResult::new_with_opt_id(
                preview, scroll_id,
            )))
        }
    }
}

#[cfg(test)]
mod buckets_endpoints {
    use crate::services::own_engine::context::OtherContext;
    use crate::services::SearcherService;

    use wrappers::bucket::FolderForm;

    use actix_web::test;

    #[test]
    async fn test_create_folder() {
        let bucket_form = FolderForm::new("test_folder");
        let other_context = OtherContext::new("test".to_string());
        let response = other_context.create_folder(&bucket_form).await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }

    #[test]
    async fn test_delete_folder() {
        let other_context = OtherContext::new("test".to_string());

        let response = other_context.delete_folder("test_folder").await;
        assert_eq!(response.status().as_u16(), 400_u16);

        let bucket_form = FolderForm::new("test_folder");

        let response = other_context.create_folder(&bucket_form).await;
        assert_eq!(response.status().as_u16(), 200_u16);

        let response = other_context.delete_folder("test_folder").await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }

    #[test]
    async fn test_get_folders() {
        let other_context = OtherContext::new("test".to_string());
        let bucket_form = FolderForm::new("test_folder");
        let response = other_context.create_folder(&bucket_form).await;
        assert_eq!(response.status().as_u16(), 200_u16);

        let response = other_context.get_all_folders().await;
        let buckets_size = response.unwrap().0.len();
        assert_eq!(buckets_size, 1);
    }

    #[test]
    async fn test_get_folder_by_id() {
        let bucket_form = FolderForm::new("test_folder");
        let other_context = OtherContext::new("test".to_string());
        let response = other_context.create_folder(&bucket_form).await;
        assert_eq!(response.status().as_u16(), 200_u16);

        let get_bucket_result = other_context.get_folder("test_folder").await;
        let bucket_uuid = &get_bucket_result.unwrap().uuid;
        assert_eq!(bucket_uuid.as_str(), "test_folder");
    }
}
