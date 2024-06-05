use crate::errors::{ErrorResponse, Successful};
use crate::errors::JsonResponse;
use crate::forms::TestExample;
use crate::forms::folders::folder::Folder;
use crate::forms::folders::forms::{CreateFolderForm, DeleteFolderForm};
use crate::services::searcher::service::FolderService;

use actix_web::{delete, get, put};
use actix_web::web::{Data, Json, Path};

type Context = Data<Box<dyn FolderService>>;

#[utoipa::path(
    get,
    path = "/storage/folders",
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
#[get("/folders")]
async fn get_folders(cxt: Context) -> JsonResponse<Vec<Folder>> {
    let client = cxt.get_ref();
    let folders = client.get_all_folders().await?;
    Ok(Json(folders))
}

#[utoipa::path(
    get,
    path = "/storage/folders/{folder_id}",
    tag = "Folders",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get details",
            example = "test-folder",
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
#[get("/folders/{folder_id}")]
async fn get_folder(cxt: Context, path: Path<String>) -> JsonResponse<Folder> {
    let client = cxt.get_ref();
    let folder = client.get_folder(path.as_ref()).await?;
    Ok(Json(folder))
}

#[utoipa::path(
    put,
    path = "/storage/folders/{folder_id}",
    tag = "Folders",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get details",
            example = "test-folder",
        )
    ),
    request_body(
        content = CreateFolderForm,
        example = json!(CreateFolderForm::test_example(None))
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful {
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
#[put("/folders/{folder_id}")]
async fn create_folder(cxt: Context, form: Json<CreateFolderForm>) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let folder_form = form.0;
    let status = client.create_folder(&folder_form).await?;
    Ok(Json(status))
}

#[utoipa::path(
    delete,
    path = "/storage/folders/{folder_id}",
    tag = "Folders",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get details",
            example = "test-folder",
        )
    ),
    request_body(
        content = DeleteFolderForm,
        example = json!(DeleteFolderForm::test_example(None))
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful {
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
#[delete("/folders/{folder_id}")]
async fn delete_folder(
    cxt: Context, 
    path: Path<String>,
    form: Json<DeleteFolderForm>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let folder_id = path.as_str();
    let folder_form = form.0;
    let status = client.delete_folder(folder_id, &folder_form).await?;
    Ok(Json(status))
}
