use crate::errors::{ErrorResponse, JsonResponse, PaginateResponse, Successful};
use crate::forms::TestExample;
use crate::forms::documents::forms::DocumentType;
use crate::forms::pagination::pagination::Paginated;
use crate::forms::pagination::forms::{DeletePaginationsForm, PaginateNextForm};
use crate::services::searcher::service::PaginatorService;

use actix_web::{delete, post};
use actix_web::web::{Data, Json, Query};
use serde_json::Value;

type Context = Data<Box<dyn PaginatorService>>;

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
#[delete("/paginate/sessions")]
async fn delete_paginate_sessions(cxt: Context, form: Json<DeletePaginationsForm>) -> JsonResponse<Successful> {
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
#[post("/paginate/next")]
async fn paginate_next(
    cxt: Context,
    form: Json<PaginateNextForm>,
    document_type: Query<DocumentType>,
) -> PaginateResponse<Vec<Value>> {
    let client = cxt.get_ref();
    let pagination_form = form.0;
    let mut founded_docs = client.paginate(&pagination_form).await?;
    
    let scroll_id = founded_docs.get_scroll_id().cloned();
    let converted = founded_docs
        .get_founded_mut()
        .iter()
        .map(|doc| document_type.to_value(doc))
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<Value>>();
    
    Ok(Json(Paginated::new_with_opt_id(converted, scroll_id)))
}
