use crate::errors::{ErrorResponse, SuccessfulResponse};
use crate::errors::{JsonResponse, PaginateResponse};
use crate::forms::folder::{Folder, FolderForm};
use crate::forms::pagination::Paginated;
use crate::forms::preview::DocumentPreview;
use crate::forms::s_params::SearchParams;
use crate::forms::TestExample;
use crate::services::service::FoldersService;

use actix_web::{delete, get, post, web, HttpResponse, ResponseError};

type Context = web::Data<Box<dyn FoldersService>>;

#[utoipa::path(
    get,
    path = "/folders/",
    tag = "Folders",
    responses(
        (
            status = 200,
            description = "Successful",
            body = [Folder],
            example = json!(vec![Folder::test_example(None)]),
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
async fn all_folders(cxt: Context) -> JsonResponse<Vec<Folder>> {
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
            example = json!(Folder::test_example(None))
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
async fn get_folder(cxt: Context, path: web::Path<String>) -> JsonResponse<Folder> {
    let client = cxt.get_ref();
    client.get_folder(path.as_str()).await
}

#[utoipa::path(
    post,
    path = "/folders/create",
    tag = "Folders",
    request_body(
        content = FolderForm,
        example = json!(FolderForm::default())
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
async fn create_folder(cxt: Context, form: web::Json<FolderForm>) -> HttpResponse {
    let client = cxt.get_ref();
    let folder_form = form.0;
    match client.create_folder(&folder_form).await {
        Ok(response) => response.to_response(),
        Err(err) => err.error_response(),
    }
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
async fn delete_folder(cxt: Context, path: web::Path<String>) -> HttpResponse {
    let client = cxt.get_ref();
    match client.delete_folder(path.as_str()).await {
        Ok(response) => response.to_response(),
        Err(err) => err.error_response(),
    }
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
            example = json!(Paginated::<Vec<DocumentPreview>>::new_with_id(
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
    cxt: Context,
    path: web::Path<String>,
    form: web::Json<SearchParams>,
) -> PaginateResponse<Vec<DocumentPreview>> {
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

            Ok(web::Json(Paginated::new_with_opt_id(preview, scroll_id)))
        }
    }
}
