use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use crate::engine::{DocumentService, FolderService, PaginatorService, SearcherService};
use crate::engine::form::{DeleteScrollsForm, DocumentTypeQuery, FulltextParams, ScrollNextForm, SemanticParams};
use crate::engine::model::{Document, Paginated};
use crate::errors::{ErrorResponse, Successful};
use crate::server::errors::{ServerError, ServerResult};
use crate::server::ServerApp;
use crate::server::swagger::SwaggerExample;
use crate::tokenizer::TokenizerService;

#[utoipa::path(
    post,
    path = "/search/fulltext",
    tag = "searcher",
    description = "Fulltext searching",
    params(
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document",
        ),
    ),
    request_body(
        content = FulltextParams,
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            // body = Paginated<Vec<Document>>,
        ),
        (
            status = 400,
            description = "Failed while searching documents",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(Some("Failed while searching documents"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(None)),
        ),
    )
)]
pub async fn search_fulltext<F, D, S, P, T>(
    State(state): State<Arc<ServerApp<F, D, S, P, T>>>,
    Query(document_type): Query<DocumentTypeQuery>,
    Json(form): Json<FulltextParams>,
) -> ServerResult<impl IntoResponse>
where
    F: FolderService + Send + Sync,
    D: DocumentService + Send + Sync,
    S: SearcherService + Send + Sync,
    P: PaginatorService + Send + Sync,
    T: TokenizerService + Send + Sync,
{
    let doc_type = document_type.get_type();
    let documents = state.searcher.search_fulltext(&form, &doc_type).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    post,
    path = "/search/semantic",
    tag = "searcher",
    description = "Semantic search by vector",
    params(
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document",
        ),
    ),
    request_body(
        content = SemanticParams,
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Vec<Document>,
        ),
        (
            status = 400,
            description = "Failed while searching tokens",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(Some("Failed while searching tokens"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(None)),
        ),
    )
)]
pub async fn search_semantic<F, D, S, P, T>(
    State(state): State<Arc<ServerApp<F, D, S, P, T>>>,
    Json(mut form): Json<SemanticParams>,
) -> ServerResult<impl IntoResponse>
where
    F: FolderService + Send + Sync,
    D: DocumentService + Send + Sync,
    S: SearcherService + Send + Sync,
    P: PaginatorService + Send + Sync,
    T: TokenizerService + Send + Sync,
{
    let query_tokens = state.tokenizer.compute(form.query()).await
        .map_err(|err| ServerError::InternalError(err.to_string()))?;

    form.set_tokens(query_tokens);

    let documents = state.searcher.search_semantic(&form).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    post,
    path = "/search/paginate/next",
    description = "Load next chunk of search results",
    tag = "searcher",
    params(
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document",
        ),
    ),
    request_body(
        content = ScrollNextForm,
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            // body = Paginated<Vec<Document>>,
        ),
        (
            status = 400,
            description = "Failed while scrolling",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(Some("Failed while scrolling"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(None)),
        ),
    )
)]
pub async fn paginate_next<F, D, S, P, T>(
    State(state): State<Arc<ServerApp<F, D, S, P, T>>>,
    Query(document_type): Query<DocumentTypeQuery>,
    Json(form): Json<ScrollNextForm>,
) -> ServerResult<impl IntoResponse>
where
    F: FolderService + Send + Sync,
    D: DocumentService + Send + Sync,
    S: SearcherService + Send + Sync,
    P: PaginatorService + Send + Sync,
    T: TokenizerService + Send + Sync,
{
    let doc_type = document_type.get_type();
    let documents = state.paginator.paginate(&form, &doc_type).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    delete,
    path = "/search/paginate/sessions",
    tag = "searcher",
    description = "Delete all existing pagination sessions",
    request_body(
        content = DeleteScrollsForm,
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
        ),
        (
            status = 400,
            description = "Failed to delete paginate session",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(Some("Failed to delete paginate session"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(None)),
        ),
    )
)]
pub async fn delete_scrolls<F, D, S, P, T>(
    State(state): State<Arc<ServerApp<F, D, S, P, T>>>,
    Json(form): Json<DeleteScrollsForm>,
) -> ServerResult<impl IntoResponse>
where
    F: FolderService + Send + Sync,
    D: DocumentService + Send + Sync,
    S: SearcherService + Send + Sync,
    P: PaginatorService + Send + Sync,
    T: TokenizerService + Send + Sync,
{
    let status = state.paginator.delete_session(&form).await?;
    Ok(Json(status))
}
