use crate::errors::{ErrorResponse, PaginateResponse};
use crate::forms::TestExample;
use crate::forms::documents::document::Document;
use crate::forms::documents::forms::DocumentType;
use crate::forms::documents::embeddings::DocumentVectors;
use crate::forms::documents::preview::DocumentPreview;
use crate::forms::documents::similar::DocumentSimilar;
use crate::forms::pagination::pagination::Paginated;
use crate::forms::searcher::s_params::SearchParams;
use crate::services::searcher::service::SearcherService;

use actix_web::{post};
use actix_web::web::{Data, Json, Query};
use serde_json::Value;

type Context = Data<Box<dyn SearcherService>>;

#[utoipa::path(
    post,
    path = "/search/fulltext",
    tag = "Search",
    request_body(
        content = SearchParams,
        example = json!(SearchParams::test_example(Some("Ocean Carrier")))
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
#[post("/fulltext")]
async fn search_fulltext(
    cxt: Context,
    form: Json<SearchParams>,
    document_type: Query<DocumentType>,
) -> PaginateResponse<Vec<Value>> {
    let client = cxt.get_ref();
    let search_form = form.0;
    let documents = client.search_fulltext(&search_form, &document_type).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    post,
    path = "/search/semantic",
    tag = "Search",
    request_body(
        content = SearchParams,
        example = json!(SearchParams::test_example(Some("Ocean Carrier")))
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
#[post("/semantic")]
async fn search_semantic(
    cxt: Context,
    form: Json<SearchParams>,
) -> PaginateResponse<Vec<DocumentVectors>> {
    let client = cxt.get_ref();
    let search_form = form.0;
    let documents = client.search_semantic(&search_form).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    post,
    path = "/search/similar",
    tag = "Search",
    request_body(
        content = SearchParams,
        example = json!(SearchParams::test_example(Some("12:JOGnP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y")))
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = [Document],
            example = json!(Paginated::<Vec<Document>>::new_with_id(
                vec![Document::test_example(None)],
                "DXF1ZXJ5QW5kRmV0Y2gBAD4WYm9laVYtZndUQlNsdDcwakFMNjU1QQ==".to_string(),
            ))
        ),
        (
            status = 400,
            description = "Failed while searching similar documents",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while searching similar documents".to_string(),
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
#[post("/similar")]
async fn search_similar(
    cxt: Context,
    form: Json<SearchParams>,
) -> PaginateResponse<Vec<DocumentSimilar>> {
    let client = cxt.get_ref();
    let search_form = form.0;
    let documents = client.search_similar(&search_form).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    post,
    path = "/folders/{folder_id}/documents",
    tag = "Search",
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
        content = SearchParams,
        example = json!(SearchParams::test_example(Some("Ocean Carrier"))),
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
#[post("/folders/{folder_id}/documents")]
async fn get_index_records(
    cxt: Data<Box<dyn SearcherService>>,
    form: Json<SearchParams>,
    document_type: Query<DocumentType>,
) -> PaginateResponse<Vec<Value>> {
    let client = cxt.get_ref();
    let search_form = form.0;
    let folder_documents = client.search_records(&search_form, &document_type).await?;
    Ok(Json(folder_documents))
}
