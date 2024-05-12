use crate::endpoints::SearcherData;
use crate::errors::{ErrorResponse, JsonResponse, PaginateJsonResponse, SuccessfulResponse};

#[cfg(feature = "enable-chunked")]
use crate::services::searcher::GroupedDocs;

use wrappers::document::Document;
use wrappers::scroll::{AllScrollsForm, NextScrollForm};

use actix_web::{delete, get, post, web, HttpResponse, ResponseError};

#[utoipa::path(
    get,
    path = "/pagination/all",
    tag = "Pagination",
    responses(
        (
            status = 200,
            description = "Successful", 
            body = [String],
            example = json!(vec![
                "DXF1ZXJ5QW5kRmV0Y2gBAAAAAAAAAD4WYm9laVYtZndUQlNsdDcwakFMNjU1QQ=="
            ])
        ),
        (
            status = 400,
            description = "Failed while getting all pagination sessions", 
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting all pagination sessions".to_string(),
            })
        ),
    )
)]
#[get("/all")]
async fn get_pagination_ids(cxt: SearcherData) -> JsonResponse<Vec<String>> {
    let client = cxt.get_ref();
    client.get_pagination_ids().await
}

#[utoipa::path(
    delete,
    path = "/pagination/",
    tag = "Pagination",
    request_body(
        content = AllScrolls,
        example = json!({
            "scroll_ids": vec![
                "DXF1ZXJ5QW5kRmV0Y2gBAAAAAAAAAD4WYm9laVYtZndUQlNsdDcwakFMNjU1QQ=="
            ]
        })
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
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
    )
)]
#[delete("/")]
async fn delete_expired_ids(cxt: SearcherData, form: web::Json<AllScrollsForm>) -> HttpResponse {
    let client = cxt.get_ref();
    let pagination_form = form.0;
    match client.delete_pagination(&pagination_form).await {
        Ok(response) => response.to_response(),
        Err(err) => err.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/pagination/next",
    tag = "Pagination",
    request_body(
        content = NextScroll,
        example = json!(NextScrollForm::new(
            "DXF1ZXJ5QW5kRmV0Y2gBAD4WYm9laVYtZndUQlNsdDcwakFMNjU1QQ==".to_string(),
            "15m".to_string(),
        ))
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
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
    )
)]
#[post("/next")]
async fn next_pagination_result(
    cxt: SearcherData,
    form: web::Json<NextScrollForm>,
) -> PaginateJsonResponse<Vec<Document>> {
    let client = cxt.get_ref();
    let pagination_form = form.0;
    client.paginate(&pagination_form).await
}

#[cfg(feature = "enable-chunked")]
#[post("/next")]
async fn next_pagination_chunked_result(
    cxt: SearcherData,
    form: web::Json<NextScrollForm>,
) -> PaginateJsonResponse<GroupedDocs> {
    let client = cxt.get_ref();
    let pagination_form = form.0;
    match client.paginate(&pagination_form).await {
        Ok(documents) => {
            let grouped = client.group_document_chunks(documents.get_founded());
            let scroll = wrappers::scroll::PaginatedResult::new(grouped);
            Ok(web::Json(scroll))
        }
        Err(err) => Err(err),
    }
}
