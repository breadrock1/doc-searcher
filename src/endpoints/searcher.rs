use crate::errors::{ErrorResponse, PaginateResponse};
use crate::forms::TestExample;
use crate::forms::documents::document::Document;
use crate::forms::documents::embeddings::DocumentVectors;
use crate::forms::documents::preview::DocumentPreview;
use crate::forms::pagination::pagination::Paginated;
use crate::forms::searcher::fulltext_params::FulltextParams;
use crate::forms::searcher::records_params::AllRecordsParams;
use crate::forms::searcher::semantic_params::SemanticParams;
use crate::forms::searcher::similar_params::SimilarParams;
use crate::forms::searcher::s_params::{SearchParams, SearchQuery};
use crate::services::searcher::service::SearcherService;

use actix_web::{post};
use actix_web::web::{Data, Json, Query};
use serde_json::Value;

type Context = Data<Box<dyn SearcherService>>;

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
    cxt: Context,
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
    cxt: Context,
    form: Json<SemanticParams>,
) -> PaginateResponse<Vec<DocumentVectors>> {
    let client = cxt.get_ref();
    let search_form = SearchParams::from(form.0);
    let documents = client.search_semantic(&search_form).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    post,
    path = "/search/similar",
    tag = "Search",
    params(
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document"
        ),
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document"
        )
    ),
    request_body(
        content = SimilarParams,
        example = json!(SimilarParams::test_example(Some("12:JOGnP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y")))
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
#[post("/similar")]
async fn search_similar(
    cxt: Context,
    form: Json<SimilarParams>,
    document_type: Query<SearchQuery>,
) -> PaginateResponse<Vec<Value>> {
    let client = cxt.get_ref();
    let search_form = SearchParams::from(form.0);
    let doc_type = document_type.0.get_type();
    let documents = client.search_similar(&search_form, &doc_type).await?;
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
