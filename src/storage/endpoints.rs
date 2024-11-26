#[cfg(feature = "enable-cacher")]
use crate::cacher::CacherService;

use crate::errors::JsonResponse;
use crate::errors::{ErrorResponse, Successful};
use crate::searcher::models::Paginated;
use crate::storage::documents::DocumentService;
use crate::storage::folders::FolderService;
use crate::storage::forms::FolderTypeQuery;
use crate::storage::forms::{CreateFolderForm, RetrieveParams, ShowAllFlag};
use crate::storage::models::{Document, Folder};
use crate::swagger::examples::TestExample;

use actix_web::web::{Data, Json, Path, Query};
use actix_web::{delete, get, post, put};
use actix_web::{web, Scope};
use serde_json::Value;

#[cfg(feature = "enable-cacher")]
type CacherRetrieveContext = Data<Box<dyn CacherService<RetrieveParams, Vec<Value>>>>;
type DocumentContext = Data<Box<dyn DocumentService>>;
type FolderContext = Data<Box<dyn FolderService>>;

pub fn build_scope() -> Scope {
    web::scope("/storage")
        .service(get_folders)
        .service(get_folder)
        .service(create_folder)
        .service(delete_folder)
        .service(get_document)
        .service(get_documents)
        .service(create_document)
        .service(delete_document)
        .service(update_document)
}

#[utoipa::path(
    get,
    path = "/storage/folders",
    tag = "Folders",
    params(
        (
            "show_all", Query,
            description = "Show all folders",
            example = "true",
        ),
    ),
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
            example = json!(ErrorResponse::test_example(Some("Failed while getting all folders"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::new(503, "Server error", "Server does not available")),
        ),
    )
)]
#[get("/folders")]
async fn get_folders(
    cxt: FolderContext,
    show_all: Query<ShowAllFlag>,
) -> JsonResponse<Vec<Folder>> {
    let client = cxt.get_ref();
    let show_all_flag = show_all.0.show_all();
    let folders = client.get_folders(show_all_flag).await?;

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
        ),
    ),
    responses(
        (
            status = 200,
            description = "Returned Folder structure",
            body = Folder,
            example = json!(Folder::test_example(None)),
        ),
        (
            status = 400,
            description = "Failed while getting folder by id",
            body = ErrorResponse,
            example = json!(ErrorResponse::test_example(Some("Failed while getting folder by id"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::new(503, "Server error", "Server does not available")),
        ),
    )
)]
#[get("/folders/{folder_id}")]
async fn get_folder(cxt: FolderContext, path: Path<String>) -> JsonResponse<Folder> {
    let client = cxt.get_ref();
    let folder = client.get_folder(path.as_ref()).await?;

    Ok(Json(folder))
}

#[utoipa::path(
    put,
    path = "/storage/folders/create",
    tag = "Folders",
    request_body(
        content = CreateFolderForm,
        example = json!(CreateFolderForm::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful::default()),
        ),
        (
            status = 400,
            description = "Failed while creating new folder",
            body = ErrorResponse,
            example = json!(ErrorResponse::test_example(Some("Failed while creating new folder"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::new(503, "Server error", "Server does not available")),
        ),
    )
)]
#[put("/folders/create")]
async fn create_folder(
    cxt: FolderContext,
    form: Json<CreateFolderForm>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let folder_form = form.0;
    let status = client.create_folder(&folder_form).await?;

    // TODO: Added creating folder into doc-watcher (cloud) service

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
        ),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful::default()),
        ),
        (
            status = 400,
            description = "Failed while deleting folder",
            body = ErrorResponse,
            example = json!(ErrorResponse::test_example(Some("Failed while deleting folder"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::new(503, "Server error", "Server does not available")),
        ),
    )
)]
#[delete("/folders/{folder_id}")]
async fn delete_folder(cxt: FolderContext, path: Path<String>) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let folder_id = path.as_str();
    let status = client.delete_folder(folder_id).await?;

    Ok(Json(status))
}

#[utoipa::path(
    get,
    path = "/storage/folders/{folder_id}/documents/{document_id}",
    tag = "Documents",
    params(
        (
            "folder_id" = &str,
            description = "Folder id where document is stored",
            example = "test-folder",
        ),
        (
            "document_id" = &str,
            description = "Document identifier to get",
            example = "<document-id>",
        ),
        (
            "folder_type", Query,
            description = "Folder type to get",
            example = "document",
        ),
    ),
    responses(
        (
            status = 200,
            description = "Returned document by id",
            body = Document,
            example = json!(Document::test_example(None)),
        ),
        (
            status = 400,
            description = "Failed while getting document",
            body = ErrorResponse,
            example = json!(ErrorResponse::test_example(Some("Failed while getting document"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::new(503, "Server error", "Server does not available")),
        ),
    )
)]
#[get("/folders/{folder_id}/documents/{document_id}")]
async fn get_document(
    cxt: DocumentContext,
    path: Path<(String, String)>,
    folder_type: Query<FolderTypeQuery>,
) -> JsonResponse<Value> {
    let client = cxt.get_ref();
    let (folder_id, doc_id) = path.as_ref();
    let folder_type = folder_type.0.folder_type();
    let document = client.get_document(folder_id, doc_id, &folder_type).await?;

    Ok(Json(document))
}

#[utoipa::path(
    put,
    path = "/storage/folders/{folder_id}/documents/create",
    tag = "Documents",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get details",
            example = "test-folder",
        ),
        (
            "folder_type", Query,
            description = "Folder type to create document",
            example = "document",
        ),
    ),
    request_body(
        content = Document,
        example = json!(Document::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful::default()),
        ),
        (
            status = 400,
            description = "Failed while creating document",
            body = ErrorResponse,
            example = json!(ErrorResponse::test_example(Some("Failed while creating document"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::new(503, "Server error", "Server does not available")),
        ),
    )
)]
#[put("/folders/{folder_id}/documents/create")]
async fn create_document(
    cxt: DocumentContext,
    form: Json<Document>,
    path: Path<String>,
    folder_type: Query<FolderTypeQuery>,
) -> JsonResponse<Successful> {
    let doc_form = form.0;
    let folder_id = path.as_ref();
    let folder_type = folder_type.0.folder_type();

    let client = cxt.get_ref();
    let status = client
        .create_document(folder_id, &doc_form, &folder_type)
        .await?;

    Ok(Json(status))
}

#[utoipa::path(
    delete,
    path = "/storage/folders/{folder_id}/documents/{document_id}",
    tag = "Documents",
    params(
        (
            "folder_id" = &str,
            description = "Folder id where documents is stored",
            example = "test-folder",
        ),
        (
            "document_id" = &str,
            description = "Document identifier to get",
            example = "98ac9896be35f47fb8442580cd9839b4",
        ),
    ),
    responses(
        (
            status = 200,
            description = "Deleted document by id",
            body = Successful,
            example = json!(Successful::default()),
        ),
        (
            status = 400,
            description = "Failed while deleting documents",
            body = ErrorResponse,
            example = json!(ErrorResponse::test_example(Some("Failed while deleting documents"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::new(503, "Server error", "Server does not available")),
        ),
    )
)]
#[delete("/folders/{folder_id}/documents/{document_id}")]
async fn delete_document(
    cxt: DocumentContext,
    path: Path<(String, String)>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let (folder_id, doc_id) = path.as_ref();
    let status = client.delete_document(folder_id, doc_id).await?;

    Ok(Json(status))
}

#[utoipa::path(
    post,
    path = "/storage/folders/{folder_id}/documents/{document_id}",
    tag = "Documents",
    params(
        (
            "folder_id" = &str,
            description = "Folder id where document is stored",
            example = "test-folder",
        ),
        (
            "document_id" = &str,
            description = "Document identifier to get",
            example = "98ac9896be35f47fb8442580cd9839b4",
        ),
        (
            "folder_type", Query,
            description = "Folder type to update document",
            example = "document",
        ),
    ),
    request_body(
        content = Document,
        example = json!(Document::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful::default()),
        ),
        (
            status = 400,
            description = "Failed while updating document",
            body = ErrorResponse,
            example = json!(ErrorResponse::test_example(Some("Failed while updating document"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::new(503, "Server error", "Server does not available")),
        ),
    )
)]
#[post("/folders/{folder_id}/documents/{document_id}")]
async fn update_document(
    cxt: DocumentContext,
    form: Json<Value>,
    path: Path<(String, String)>,
    folder_type: Query<FolderTypeQuery>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let (folder_id, _) = path.as_ref();
    let folder_type = folder_type.0.folder_type();
    let status = client
        .update_document(folder_id, &form.0, &folder_type)
        .await?;

    Ok(Json(status))
}

#[utoipa::path(
    post,
    path = "/storage/folders/{folder_id}/documents",
    tag = "Documents",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get stored documents",
            example = "test-folder",
        ),
        (
            "folder_type", Query,
            description = "Folder type to retrieve",
            example = "document",
        ),
    ),
    request_body(
        content = RetrieveParams,
        example = json!(RetrieveParams::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Paginated::<Vec<Document>>,
            example = json!(Paginated::<Vec<Document>>::test_example(None)),
        ),
        (
            status = 400,
            description = "Failed while getting all records",
            body = ErrorResponse,
            example = json!(ErrorResponse::test_example(Some("Failed while getting all records"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::new(503, "Server error", "Server does not available")),
        ),
    )
)]
#[post("/folders/{folder_id}/documents")]
async fn get_documents(
    cxt: DocumentContext,
    #[cfg(feature = "enable-cacher")] cacher: CacherRetrieveContext,
    path: Path<String>,
    form: Json<RetrieveParams>,
    folder_type: Query<FolderTypeQuery>,
) -> JsonResponse<Vec<Value>> {
    let client = cxt.get_ref();
    let folder_id = path.as_ref();
    let params = form.0;

    #[cfg(feature = "enable-cacher")]
    if let Some(docs) = cacher.load(&params).await {
        tracing::info!("loaded from cache by params: {:?}", &params);
        return Ok(Json(docs));
    }

    let folder_type = folder_type.0.folder_type();
    let documents = client
        .get_documents(folder_id, &folder_type, &params)
        .await?;

    #[cfg(feature = "enable-cacher")]
    cacher.insert(&params, &documents).await;

    Ok(Json(documents))
}
