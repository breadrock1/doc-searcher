use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use doc_search_core::domain::searcher::models::{
    ResultParams, RetrieveIndexDocumentsParams, SearchKindParams, SearchingParams,
};
use doc_search_core::domain::searcher::{IPaginator, ISearcher};
use doc_search_core::domain::storage::models::LargeDocument;
use doc_search_core::domain::storage::{IDocumentPartStorage, IIndexStorage};
use std::sync::Arc;

use crate::server::httpserver::api::v1::form::{CreateDocumentForm, RetrieveDocumentForm};
use crate::server::httpserver::api::v1::query::CreateDocumentQuery;
use crate::server::httpserver::api::v1::schema::{DocumentPartSchema, StoredDocumentSchema};
use crate::server::httpserver::api::v1::swagger::*;
use crate::server::httpserver::ServerApp;
use crate::server::{ServerResult, Success};

pub const STORAGE_ALL_DOCUMENTS_URL: &str = "/storage/{index_ids}/documents";
pub const STORAGE_DOCUMENT_URL: &str = "/storage/{index_id}/{document_id}";
pub const CREATE_DOCUMENT_URL: &str = "/storage/{index_id}/create";

const RETRIEVE_DESCRIPTION: &str =
    include_str!("../../../../../../docs/swagger/swagger-ui-retrieve");

const CREATE_DOCUMENT_DESCRIPTION: &str =
    include_str!("../../../../../../docs/swagger/swagger-ui-create-doc");

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
            "large_document_id" = &str,
            description = "Document id to load information",
            example = "c5cdd3bfad598ec73dc5fe83fecbba3e",
        ),
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Document object stored into index",
            body = Vec<DocumentPartSchema>,
        ),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn get_document_parts<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(path): Path<(String, String)>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone + 'static,
{
    let (folder_id, large_doc_id) = path;
    let storage = state.get_storage();
    let document = storage
        .get_all_document_parts(&folder_id, &large_doc_id)
        .await?;
    let response = document
        .into_iter()
        .filter_map(|it| it.try_into().ok())
        .collect::<Vec<DocumentPartSchema>>();

    Ok(Json(response))
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
            body = Vec<DocumentPartSchema>,
        ),
        (status = 400, description = "Validation form error"),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn get_index_documents<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(index_ids): Path<String>,
    Json(form): Json<RetrieveDocumentForm>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone + 'static,
{
    let result = ResultParams {
        size: 100,
        offset: 0,
        ..Default::default()
    };

    let params = RetrieveIndexDocumentsParams::try_from(form)?;
    let indexes = index_ids
        .split(',')
        .map(String::from)
        .collect::<Vec<String>>();
    let params = SearchingParams::new(indexes, SearchKindParams::Retrieve(params), result, None);

    let searcher = state.get_searcher();
    let documents = searcher.search_document_parts(&params).await?;
    let response = documents
        .founded
        .into_iter()
        .filter_map(|it| it.document.try_into().ok())
        .collect::<Vec<DocumentPartSchema>>();

    Ok(Json(response))
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
    Searcher: ISearcher + IPaginator + Send + Sync + Clone + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone + 'static,
{
    let documents = form
        .into_iter()
        .filter_map(|it| it.try_into().ok())
        .collect::<Vec<LargeDocument>>();

    let storage = state.get_storage();
    let stored_documents_info = storage
        .store_documents(&index_id, documents)
        .await?
        .into_iter()
        .filter_map(|it| it.try_into().ok())
        .collect::<Vec<StoredDocumentSchema>>();

    Ok(Json(stored_documents_info))
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
    Json(form): Json<CreateDocumentForm>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone + 'static,
{
    let is_force = query.force.unwrap_or(false);
    let storage = state.get_storage();
    let document = form.try_into()?;
    let stored_doc = storage
        .store_document(&index_id, document, is_force)
        .await?;

    let response: StoredDocumentSchema = stored_doc.try_into()?;
    Ok((StatusCode::CREATED, Json(response)))
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
    Searcher: ISearcher + IPaginator + Send + Sync + Clone + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone + 'static,
{
    let (folder_id, doc_id) = path;
    let storage = state.get_storage();
    storage.delete_document(&folder_id, &doc_id).await?;
    let status = Success::default();
    Ok(Json(status))
}
