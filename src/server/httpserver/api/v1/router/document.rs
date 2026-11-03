use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use doc_search_core::domain::searcher::models::RetrieveIndexDocumentsParamsBuilder;
use doc_search_core::domain::searcher::models::{FilterParams, SearchKindParams, SearchingParams};
use doc_search_core::domain::searcher::{IPaginator, ISearcher};
use doc_search_core::domain::storage::models::LargeDocument;
use doc_search_core::domain::storage::{IDocumentPartStorage, IIndexStorage};
use doc_search_core::shared::kernel::{IndexId, LargeDocumentId};

use crate::server::httpserver::api::v1::form::{CreateDocumentForm, RetrieveDocumentForm};
use crate::server::httpserver::api::v1::query::CreateDocumentQuery;
use crate::server::httpserver::api::v1::router::{join_path_prefix, resolve_resource_info};
use crate::server::httpserver::api::v1::schema::{DocumentPartSchema, StoredDocumentSchema};
use crate::server::httpserver::swagger::DefaultErrorForm;
use crate::server::httpserver::ServerApp;
use crate::server::{ServerError, ServerResult, Success};

pub const STORAGE_ALL_DOCUMENTS_URL: &str = "/storage/{resource_id}/documents";
pub const STORAGE_DOCUMENT_URL: &str = "/storage/{resource_id}/{document_id}";
pub const STORAGE_GET_DOCUMENT_PARTS_URL: &str = "/storage/{resource_id}/{large_document_id}";
pub const CREATE_DOCUMENT_URL: &str = "/storage/{resource_id}/create";

const RETRIEVE_DESCRIPTION: &str = include_str!("../../../swagger/descriptions/searcher-retrieve");
const CREATE_DOC_DESCRIPTION: &str = include_str!("../../../swagger/descriptions/document-create");

#[utoipa::path(
    get,
    tag = "document",
    path = STORAGE_GET_DOCUMENT_PARTS_URL,
    description = "Load full Document information by id",
    params(
        (
            "resource_id" = &str,
            description = "Resource id (uuid) that represent index to get document",
            example = "53079efa-b0f2-4058-96dc-c792531972db",
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
            description = "Document object stored into the resource index",
            body = Vec<DocumentPartSchema>,
        ),
        (status = 400, description = "Invalid resource_id"),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Resource or Document not found"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn get_document_parts<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(path): Path<(String, String)>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    let (resource_id, large_doc_id) = path;
    let res_manager = state.get_resource_manager().clone();
    let info = resolve_resource_info(res_manager, &resource_id).await?;

    let index_id = IndexId(info.index);
    let large_doc_id = LargeDocumentId(large_doc_id);
    let storage = state.get_storage();
    let document = storage
        .get_all_document_parts(&index_id, &large_doc_id)
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
            "resource_id" = &str,
            description = "Resource id (uuid) that represent index to get document",
            example = "53079efa-b0f2-4058-96dc-c792531972db",
        ),
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "List of retrieved documents stored in the resource index",
            body = Vec<DocumentPartSchema>,
        ),
        (status = 400, description = "Validation form error"),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Resource not found"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn get_index_documents<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(resource_id): Path<String>,
    Json(form): Json<RetrieveDocumentForm>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    let res_manager = state.get_resource_manager().clone();
    let info = resolve_resource_info(res_manager, &resource_id).await?;

    let mut form = form;
    form.path = join_path_prefix(&info.prefix, form.path.as_deref())?;

    let indexes = vec![info.index];
    let result = form.result.try_into()?;
    let filter_params = form
        .filter
        .map(|it| FilterParams::try_from(it).ok())
        .unwrap_or_default();

    let retrieve_params = RetrieveIndexDocumentsParamsBuilder::default()
        .path(form.path)
        .build()
        .map_err(|err| ServerError::IncorrectInputForm(err.to_string()))?;

    let params = SearchingParams::new(
        indexes,
        SearchKindParams::Retrieve(retrieve_params),
        result,
        filter_params,
    );

    let searcher = state.get_searcher();
    let documents = searcher.search_document_parts(params).await?;
    let response = documents
        .founded
        .into_iter()
        .filter_map(|it| {
            let index = it.index;
            let converted_doc: Option<DocumentPartSchema> = it.document.try_into().ok();
            match converted_doc {
                None => None,
                Some(mut doc) => {
                    doc.index = Some(index);
                    Some(doc)
                }
            }
        })
        .collect::<Vec<DocumentPartSchema>>();

    Ok(Json(response))
}

#[utoipa::path(
    put,
    tag = "document",
    path = STORAGE_ALL_DOCUMENTS_URL,
    description = "Store array of documents into the resource index",
    request_body(content = Vec<CreateDocumentForm>),
    params(
        (
            "resource_id" = &str,
            description = "Resource id (uuid) that represent index to get document",
            example = "53079efa-b0f2-4058-96dc-c792531972db",
        ),
    ),
    responses(
        (
            status = 201,
            content_type="application/json",
            description = "List of documents stored into the resource index",
            body = Vec<StoredDocumentSchema>,
        ),
        (status = 400, description = "Validation form error"),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Resource not found"),
        (status = 409, description = "Store documents conflict"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn store_documents<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(resource_id): Path<String>,
    Json(form): Json<Vec<CreateDocumentForm>>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    let res_manager = state.get_resource_manager().clone();
    let info = resolve_resource_info(res_manager, &resource_id).await?;

    let mut form = form;
    for document in form.iter_mut() {
        let joined = join_path_prefix(&info.prefix, Some(document.file_path.as_str()))?;
        if let Some(joined) = joined {
            document.file_path = joined;
        }
    }

    let index_id = IndexId(info.index);
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

    Ok((StatusCode::CREATED, Json(stored_documents_info)))
}

#[utoipa::path(
    put,
    tag = "document",
    path = CREATE_DOCUMENT_URL,
    description = CREATE_DOC_DESCRIPTION,
    request_body(content = CreateDocumentForm),
    params(
        (
            "resource_id" = &str,
            description = "Resource id (uuid) that represent index to get document",
            example = "53079efa-b0f2-4058-96dc-c792531972db",
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
        (status = 404, description = "Resource not found"),
        (status = 409, description = "Store document conflict"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn store_document<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(resource_id): Path<String>,
    Query(query): Query<CreateDocumentQuery>,
    Json(form): Json<CreateDocumentForm>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    let res_manager = state.get_resource_manager().clone();
    let info = resolve_resource_info(res_manager, &resource_id).await?;

    let mut form = form;
    let joined = join_path_prefix(&info.prefix, Some(form.file_path.as_str()))?;
    if let Some(joined) = joined {
        form.file_path = joined;
    }

    let index_id = IndexId(info.index);
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
    tag = "document",
    path = STORAGE_DOCUMENT_URL,
    description = "Delete Document object from the resource index",
    params(
        (
            "resource_id" = &str,
            description = "Resource id (uuid) that represent index to get document",
            example = "53079efa-b0f2-4058-96dc-c792531972db",
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
        (status = 400, description = "Invalid resource_id"),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Resource or Document not found"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn delete_document<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(path): Path<(String, String)>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    let (resource_id, large_doc_id) = path;
    let res_manager = state.get_resource_manager().clone();
    let info = resolve_resource_info(res_manager, &resource_id).await?;

    let index_id = IndexId(info.index);
    let large_doc_id = LargeDocumentId(large_doc_id);
    let storage = state.get_storage();
    storage.delete_document(&index_id, &large_doc_id).await?;
    let status = Success::default();
    Ok(Json(status))
}
