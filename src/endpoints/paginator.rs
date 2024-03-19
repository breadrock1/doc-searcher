use crate::endpoints::SearcherData;
use crate::errors::{JsonResponse, PaginateJsonResponse};

use actix_web::{delete, get, post, web, HttpResponse};
use wrappers::document::Document;
use wrappers::scroll::{AllScrolls, NextScroll};

#[get("/")]
async fn get_pagination_ids(cxt: SearcherData) -> JsonResponse<Vec<String>> {
    let client = cxt.get_ref();
    client.get_pagination_ids().await
}

#[delete("/")]
async fn delete_expired_ids(cxt: SearcherData, form: web::Json<AllScrolls>) -> HttpResponse {
    let client = cxt.get_ref();
    let pagination_form = form.0;
    client.delete_pagination_ids(&pagination_form).await
}

#[post("/next")]
async fn next_pagination_result(
    cxt: SearcherData,
    form: web::Json<NextScroll>,
) -> PaginateJsonResponse<Vec<Document>> {
    let client = cxt.get_ref();
    let pagination_form = form.0;
    client.next_pagination_result(&pagination_form).await
}

#[cfg(feature = "enable-chunked")]
#[post("/next")]
async fn next_pagination_chunked_result(
    cxt: SearcherData,
    form: web::Json<NextScroll>,
) -> PaginateJsonResponse<crate::services::GroupedDocs> {
    let client = cxt.get_ref();
    let pagination_form = form.0;
    match client.next_pagination_result(&pagination_form).await {
        Ok(documents) => {
            let grouped = client.group_document_chunks(documents.get_founded());
            let scroll = wrappers::scroll::PagintatedResult::new(grouped);
            Ok(web::Json(scroll))
        }
        Err(err) => Err(err),
    }
}
