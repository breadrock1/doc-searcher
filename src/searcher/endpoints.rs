use crate::errors::{ErrorResponse, JsonResponse, PaginateResponse, Successful};
use crate::searcher::forms::{AllRecordsParams, DeletePaginationsForm, FulltextParams, PaginateNextForm, SearchQuery, SemanticParams};
use crate::searcher::models::{Paginated, SearchParams};
use crate::searcher::{PaginatorService, SearcherService};
use crate::storage::models::{Document, DocumentPreview, DocumentVectors};
use crate::storage::forms::DocumentType;
use crate::swagger::examples::TestExample;

use actix_web::{delete, post, Scope, web};
use actix_web::web::{Data, Json, Query};
use serde_json::Value;

type SearchContext = Data<Box<dyn SearcherService>>;
type PaginateContext = Data<Box<dyn PaginatorService>>;

pub fn build_scope() -> Scope {
    web::scope("/search")
        .service(search_fulltext)
        .service(search_semantic)
        .service(delete_paginate_sessions)
        .service(paginate_next)
}

#[utoipa::path(
    post,
    path = "/search/fulltext",
    tag = "Search",
    params(
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document"
        )
    ),
    request_body(
        content = FulltextParams,
        example = json!(FulltextParams::test_example(Some("Ocean Carrier")))
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Paginated<Vec<Document>>,
            example = json!(Paginated::<Vec<Document>>::test_example(None))
        ),
        (
            status = 400,
            description = "Failed while searching documents",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while searching documents".to_string(),
                attachments: None,
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
                attachments: None,
            })
        )
    )
)]
#[post("/fulltext")]
async fn search_fulltext(
    cxt: SearchContext,
    form: Json<FulltextParams>,
    document_type: Query<SearchQuery>,
) -> PaginateResponse<Vec<Value>> {
    let client = cxt.get_ref();
    let search_form = SearchParams::from(form.0);
    let doc_type = document_type.0.get_type();
    let documents = client.search_fulltext(&search_form, &doc_type).await?;
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
            example = "document"
        ),
    ),
    request_body(
        content = SemanticParams,
        example = json!(SemanticParams::test_example(Some("Ocean Carrier")))
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = [Document],
            example = json!(Paginated::<Vec<DocumentVectors>>::test_example(None))
        ),
        (
            status = 400,
            description = "Failed while searching tokens",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while searching tokens".to_string(),
                attachments: None,
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
                attachments: None,
            })
        )
    )
)]
#[post("/semantic")]
async fn search_semantic(
    cxt: SearchContext,
    form: Json<SemanticParams>,
    document_type: Query<SearchQuery>,
) -> PaginateResponse<Vec<Value>> {
    let client = cxt.get_ref();
    let search_form = SearchParams::from(form.0);
    let doc_type = document_type.0.get_type();
    let documents = client.search_semantic(&search_form, &doc_type).await?;
    Ok(Json(documents))
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
            "document_type", Query,
            description = "Document type to convert",
            example = "document"
        )
    ),
    request_body(
        content = AllRecordsParams,
        example = json!(AllRecordsParams::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = PaginatedResult<Vec<Document>>,
            example = json!(Paginated::<Vec<DocumentPreview>>::test_example(None)),
        ),
        (
            status = 400,
            description = "Failed while getting stored documents into folder",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting stored documents into folder".to_string(),
                attachments: None,
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
                attachments: None,
            })
        )
    )
)]
#[post("/folders/{folder_id}/documents")]
async fn get_index_records(
    cxt: Data<Box<dyn SearcherService>>,
    form: Json<AllRecordsParams>,
    document_type: Query<SearchQuery>,
) -> PaginateResponse<Vec<Value>> {
    let client = cxt.get_ref();
    let search_form = SearchParams::from(form.0);
    let doc_type = document_type.0.get_type();
    let folder_documents = client.search_records(&search_form, &doc_type).await?;
    Ok(Json(folder_documents))
}

#[utoipa::path(
    delete,
    path = "/search/paginate/sessions",
    tag = "Search",
    request_body(
        content = DeletePaginationsForm,
        example = json!(DeletePaginationsForm::test_example(None))
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful {
                code: 200,
                message: "Done".to_string(),
            })
        ),
        (
            status = 400,
            description = "Failed while deleting pagination sessions",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while deleting pagination sessions".to_string(),
                attachments: None,
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
                attachments: None,
            })
        )
    )
)]
#[delete("/paginate/sessions")]
async fn delete_paginate_sessions(
    cxt: PaginateContext,
    form: Json<DeletePaginationsForm>,
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
            example = "document"
        )
    ),
    request_body(
        content = PaginateNextForm,
        example = json!(PaginateNextForm::test_example(None))
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful {
                code: 200,
                message: "Done".to_string(),
            })
        ),
        (
            status = 400,
            description = "Failed while scrolling",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while scrolling".to_string(),
                attachments: None,
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
                attachments: None,
            })
        )
    )
)]
#[post("/paginate/next")]
async fn paginate_next(
    cxt: PaginateContext,
    form: Json<PaginateNextForm>,
    document_type: Query<DocumentType>,
) -> PaginateResponse<Vec<Value>> {
    let client = cxt.get_ref();
    let pag_form = form.0;
    let founded_docs = client.paginate(&pag_form, &document_type).await?;
    Ok(Json(founded_docs))
}
