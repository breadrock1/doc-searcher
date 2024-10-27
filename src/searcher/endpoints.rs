#[cfg(feature = "enable-cacher")]
use crate::cacher::CacherService;

use crate::embeddings::EmbeddingsService;
use crate::errors::{ErrorResponse, JsonResponse, PaginateResponse, Successful};
use crate::searcher::forms::{DeleteScrollsForm, DocumentTypeQuery, ScrollNextForm};
use crate::searcher::forms::{FulltextParams, SemanticParams};
use crate::searcher::models::Paginated;
use crate::searcher::{PaginatorService, SearcherService};
use crate::storage::models::{Document, DocumentVectors};
use crate::swagger::examples::TestExample;

use actix_web::web::{Data, Json, Query};
use actix_web::{delete, post, web, Scope};
use serde_json::Value;

type SearchContext = Data<Box<dyn SearcherService>>;
type PaginateContext = Data<Box<dyn PaginatorService>>;
type EmbeddingsContext = Data<Box<dyn EmbeddingsService>>;

#[cfg(feature = "enable-cacher")]
type CacherFulltextContext = Data<Box<dyn CacherService<FulltextParams, Paginated<Vec<Value>>>>>;
#[cfg(feature = "enable-cacher")]
type CacherSemanticContext = Data<Box<dyn CacherService<SemanticParams, Paginated<Vec<Value>>>>>;
#[cfg(feature = "enable-cacher")]
type CacherPaginateContext = Data<Box<dyn CacherService<ScrollNextForm, Paginated<Vec<Value>>>>>;

pub fn build_scope() -> Scope {
    let scope = web::scope("/search")
        .service(search_fulltext)
        .service(delete_scrolls)
        .service(paginate_next);

    #[cfg(feature = "enable-semantic")]
    let scope = scope.service(search_semantic);

    #[allow(clippy::let_and_return)]
    scope
}

#[utoipa::path(
    post,
    path = "/search/fulltext",
    tag = "Search",
    params(
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document",
        ),
    ),
    request_body(
        content = FulltextParams,
        example = json!(FulltextParams::test_example(Some("Ocean Carrier"))),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Paginated<Vec<Document>>,
            example = json!(Paginated::<Vec<Document>>::test_example(None)),
        ),
        (
            status = 400,
            description = "Failed while searching documents",
            body = ErrorResponse,
            example = json!(ErrorResponse::test_example(Some("Failed while searching documents"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::new(503, "Server error", "Server does not available")),
        ),
    )
)]
#[post("/fulltext")]
async fn search_fulltext(
    cxt: SearchContext,
    #[cfg(feature = "enable-cacher")] cacher: CacherFulltextContext,
    form: Json<FulltextParams>,
    document_type: Query<DocumentTypeQuery>,
) -> PaginateResponse<Vec<Value>> {
    let client = cxt.get_ref();
    let search_form = form.0;

    #[cfg(feature = "enable-cacher")]
    if let Some(docs) = cacher.load(&search_form).await {
        tracing::info!("loaded from cache by params: {:?}", &search_form);
        return Ok(Json(docs));
    }

    let doc_type = document_type.0.get_type();
    let documents = client.search_fulltext(&search_form, &doc_type).await?;

    #[cfg(feature = "enable-cacher")]
    cacher.insert(&search_form, &documents).await;

    Ok(Json(documents))
}

#[utoipa::path(
    post,
    path = "/search/semantic",
    tag = "Search",
    params(
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document",
        ),
    ),
    request_body(
        content = SemanticParams,
        example = json!(SemanticParams::test_example(Some("Ocean Carrier"))),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = [Document],
            example = json!(Paginated::<Vec<DocumentVectors>>::test_example(None)),
        ),
        (
            status = 400,
            description = "Failed while searching tokens",
            body = ErrorResponse,
            example = json!(ErrorResponse::test_example(Some("Failed while searching tokens"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::new(503, "Server error", "Server does not available")),
        ),
    )
)]
#[post("/semantic")]
async fn search_semantic(
    cxt: SearchContext,
    #[cfg(feature = "enable-cacher")] cacher: CacherSemanticContext,
    embed: EmbeddingsContext,
    form: Json<SemanticParams>,
) -> PaginateResponse<Vec<Value>> {
    let client = cxt.get_ref();

    let mut search_form = form.0;
    let query_tokens = embed.load_from_text(search_form.query()).await?;

    search_form.set_tokens(query_tokens);

    #[cfg(feature = "enable-cacher")]
    if let Some(docs) = cacher.load(&search_form).await {
        tracing::info!("loaded from cache by params: {:?}", &search_form);
        return Ok(Json(docs));
    }

    let documents = client.search_semantic(&search_form).await?;

    #[cfg(feature = "enable-cacher")]
    cacher.insert(&search_form, &documents).await;

    Ok(Json(documents))
}

#[utoipa::path(
    delete,
    path = "/search/paginate/sessions",
    tag = "Search",
    request_body(
        content = DeleteScrollsForm,
        example = json!(DeleteScrollsForm::test_example(None)),
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
            description = "Failed to delete paginate session",
            body = ErrorResponse,
            example = json!(ErrorResponse::test_example(Some("Failed to delete paginate session"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::new(503, "Server error", "Server does not available")),
        ),
    )
)]
#[delete("/paginate/sessions")]
async fn delete_scrolls(
    cxt: PaginateContext,
    form: Json<DeleteScrollsForm>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let pagination_form = form.0;
    let status = client.delete_session(&pagination_form).await?;
    Ok(Json(status))
}

#[utoipa::path(
    post,
    path = "/search/paginate/next",
    tag = "Search",
    params(
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document",
        ),
    ),
    request_body(
        content = ScrollNextForm,
        example = json!(ScrollNextForm::test_example(None))
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
            description = "Failed while scrolling",
            body = ErrorResponse,
            example = json!(ErrorResponse::test_example(Some("Failed while scrolling"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::new(503, "Server error", "Server does not available")),
        ),
    )
)]
#[post("/paginate/next")]
async fn paginate_next(
    cxt: PaginateContext,
    #[cfg(feature = "enable-cacher")] cacher: CacherPaginateContext,
    form: Json<ScrollNextForm>,
    document_type: Query<DocumentTypeQuery>,
) -> PaginateResponse<Vec<Value>> {
    let client = cxt.get_ref();
    let pag_form = form.0;

    #[cfg(feature = "enable-cacher")]
    if let Some(docs) = cacher.load(&pag_form).await {
        tracing::info!("loaded from cache by paginate form: {:?}", &pag_form);
        return Ok(Json(docs));
    }

    let doc_type = document_type.0.get_type();
    let documents = client.paginate(&pag_form, &doc_type).await?;

    #[cfg(feature = "enable-cacher")]
    cacher.insert(&pag_form, &documents).await;

    Ok(Json(documents))
}
