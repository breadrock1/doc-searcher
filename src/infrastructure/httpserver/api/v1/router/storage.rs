use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::sync::Arc;

use crate::application::services::server::{ServerResult, Success};
use crate::application::services::storage::{
    DocumentManager, DocumentSearcher, IndexManager, PaginateManager,
};
use crate::application::structures::params::{CreateIndexParams, RetrieveDocumentParams};
use crate::application::structures::{DocumentPart, UserInfo};
use crate::infrastructure::httpserver::api::v1::models::{
    CreateDocumentForm, CreateDocumentQuery, CreateIndexForm, DocumentSchema, IndexSchema,
    RetrieveDocumentForm, StoredDocumentSchema, UpdateDocumentForm,
};
use crate::infrastructure::httpserver::api::v1::swagger::*;
use crate::infrastructure::httpserver::mw::header::UserInfoHeader;
use crate::infrastructure::httpserver::ServerApp;

pub const STORAGE_ALL_INDEXES_URL: &str = "/storage/indexes";
pub const STORAGE_INDEX_URL: &str = "/storage/{index_id}";
pub const STORAGE_ALL_DOCUMENTS_URL: &str = "/storage/{index_ids}/documents";
pub const STORAGE_DOCUMENT_URL: &str = "/storage/{index_id}/{document_id}";
pub const CREATE_DOCUMENT_URL: &str = "/storage/{index_id}/create";

const RETRIEVE_DESCRIPTION: &str =
    include_str!("../../../../../../docs/swagger/swagger-ui-retrieve.description");
const CREATE_DOCUMENT_DESCRIPTION: &str =
    include_str!("../../../../../../docs/swagger/swagger-ui-create-doc.description");

#[utoipa::path(
    get,
    tag = "index",
    path = STORAGE_ALL_INDEXES_URL,
    description = "Get all existing indexes",
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "List of all exists indexes",
            body = Vec<IndexSchema>,
        ),
        (status = 400, description = "Failed to get all indexes"),
        (status = 401, description = "Unauthorized access"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn get_all_indexes<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Extension(user): Extension<Option<UserInfoHeader>>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let user_info = user.as_ref().map(UserInfo::from);
    let storage = state.get_storage();
    let indexes = storage.get_all_indexes(user_info.as_ref()).await?;

    let indexes_schema = indexes
        .into_iter()
        .map(|it| it.into())
        .collect::<Vec<IndexSchema>>();

    Ok(Json(indexes_schema))
}

#[utoipa::path(
    get,
    tag = "index",
    path = STORAGE_INDEX_URL,
    description = "Get index information by id",
    params(
        (
            "index_id" = &str,
            description = "Index id to get information",
            example = "test-folder",
        ),
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Index information",
            body = IndexSchema,
        ),
        (status = 400, description = "Failed to get index information"),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Index not found"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn get_index<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(index_id): Path<String>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let storage = state.get_storage();
    let folder = storage.get_index(&index_id).await?;
    Ok(Json(folder))
}

#[utoipa::path(
    put,
    tag = "index",
    path = STORAGE_INDEX_URL,
    description = "Create new index",
    request_body(content = CreateIndexForm),
    params(
        (
            "index_id" = &str,
            description = "Index id to create",
            example = "test-folder",
        ),
    ),
    responses(
        (
            status = 201,
            content_type="application/json",
            description = "Index has been created",
            body = String,
            example = "test-folder"
        ),
        (status = 400, description = "Validation form error"),
        (status = 401, description = "Unauthorized access"),
        (status = 409, description = "Conflict while creating index"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn create_index<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(index_id): Path<String>,
    Json(form): Json<CreateIndexForm>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let mut params = CreateIndexParams::try_from(form)?;
    params.set_id(index_id);

    let storage = state.get_storage();
    let index_id = storage.create_index(&params).await?;
    let status = Success::new(201, &index_id);
    Ok((StatusCode::CREATED, Json(status)))
}

#[utoipa::path(
    delete,
    tag = "index",
    path = STORAGE_INDEX_URL,
    description = "Delete existing index by id",
    params(
        (
            "index_id" = &str,
            description = "Delete existing index by id",
            example = "test-folder",
        ),
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Index has been deleted",
            body = Success
        ),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn delete_index<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(index_id): Path<String>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let storage = state.get_storage();
    storage.delete_index(&index_id).await?;
    let status = Success::default();
    Ok(Json(status))
}

#[utoipa::path(
    get,
    tag = "document",
    path = STORAGE_DOCUMENT_URL,
    description = "Load full Document information by id",
    params(
        (
            "index_id" = &str,
            description = "Index id where is stored Document",
            example = "test-folder",
        ),
        (
            "document_id" = &str,
            description = "Document id to load information",
            example = "c5cdd3bfad598ec73dc5fe83fecbba3e",
        ),
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Document object stored into index",
            body = DocumentSchema,
        ),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn get_document<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(path): Path<(String, String)>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let (folder_id, doc_id) = path;
    let storage = state.get_storage();
    let document = storage.get_document(&folder_id, &doc_id).await?;
    Ok(Json(document))
}

#[utoipa::path(
    post,
    tag = "document",
    path = STORAGE_ALL_DOCUMENTS_URL,
    description = RETRIEVE_DESCRIPTION,
    request_body(content = RetrieveDocumentForm),
    params(
        (
            "index_ids" = &str,
            description = "Index id's to retrieve documents",
            example = "test-folder",
        ),
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "List of retrieved documents stored into passed index id",
            body = Vec<DocumentSchema>,
        ),
        (status = 400, description = "Validation form error"),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn get_documents<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(index_ids): Path<String>,
    Json(form): Json<RetrieveDocumentForm>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let params = RetrieveDocumentParams::try_from(form)?;
    let searcher = state.get_searcher();
    let documents = searcher.retrieve(&index_ids, &params).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    put,
    tag = "document",
    path = STORAGE_ALL_DOCUMENTS_URL,
    description = "Store array of documents into index",
    request_body(content = Vec<CreateDocumentForm>),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "List of documents to store into passed index id",
            body = Vec<StoredDocumentSchema>,
        ),
        (status = 400, description = "Validation form error"),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Index not found"),
        (status = 409, description = "Store documents conflict"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn store_documents<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(index_id): Path<String>,
    Json(form): Json<Vec<CreateDocumentForm>>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let documents = form
        .into_iter()
        .filter_map(|it| it.try_into().ok())
        .collect::<Vec<DocumentPart>>();

    let storage = state.get_storage();
    let documents = storage
        .store_documents(&index_id, documents.as_slice())
        .await?
        .into_iter()
        .map(|it| it.into())
        .collect::<Vec<StoredDocumentSchema>>();

    Ok(Json(documents))
}

#[utoipa::path(
    put,
    tag = "document",
    path = CREATE_DOCUMENT_URL,
    description = CREATE_DOCUMENT_DESCRIPTION,
    request_body(content = CreateDocumentForm),
    params(
        (
            "index_id" = &str,
            description = "Index id to store Document object",
            example = "test-folder",
        ),
        CreateDocumentQuery,
    ),

    responses(
        (
            status = 201,
            content_type="application/json",
            description = "Document has been stored successful",
            body = Success,
            example = json!(Success::new(201, "c5cdd3bfad598ec73dc5fe83fecbba3e")),
        ),
        (status = 400, description = "Validation form error"),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Index not found"),
        (status = 409, description = "Store document conflict"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn store_document<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(index_id): Path<String>,
    Query(query): Query<CreateDocumentQuery>,
    Json(form): Json<DocumentSchema>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let is_force = query.force.unwrap_or(false);
    let storage = state.get_storage();
    let stored_doc = storage
        .store_document(&index_id, &form.into(), is_force)
        .await?;

    let status = Success::new(201, &stored_doc.id);
    Ok((StatusCode::CREATED, Json(status)))
}

#[utoipa::path(
    delete,
    path = STORAGE_DOCUMENT_URL,
    tag = "document",
    description = "Delete Document object from index",
    params(
        (
            "index_id" = &str,
            description = "Index id where is stored Document",
            example = "test-folder",
        ),
        (
            "document_id" = &str,
            description = "Document id to delete it",
            example = "c5cdd3bfad598ec73dc5fe83fecbba3e",
        ),
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Deleted document by id",
            body = Success,
        ),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Index or Document not found"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn delete_document<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(path): Path<(String, String)>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let (folder_id, doc_id) = path;
    let storage = state.get_storage();
    storage.delete_document(&folder_id, &doc_id).await?;
    let status = Success::default();
    Ok(Json(status))
}

#[utoipa::path(
    patch,
    tag = "document",
    path = STORAGE_DOCUMENT_URL,
    description = "Update existing Document object",
    request_body(content = UpdateDocumentForm),
    params(
        (
            "index_id" = &str,
            description = "Index id where is stored Document",
            example = "test-folder",
        ),
        (
            "document_id" = &str,
            description = "Document id to update it",
            example = "c5cdd3bfad598ec73dc5fe83fecbba3e",
        ),
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Document has been updated successful",
            body = Success,
        ),
        (status = 400, description = "Validation form error"),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Index or Document not found"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn update_document<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(path): Path<(String, String)>,
    Json(form): Json<UpdateDocumentForm>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let (folder_id, doc_id) = path;
    let storage = state.get_storage();
    storage
        .update_document(&folder_id, &doc_id, &form.try_into()?)
        .await?;
    let status = Success::default();
    Ok(Json(status))
}
