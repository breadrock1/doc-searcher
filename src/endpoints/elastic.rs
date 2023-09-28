use crate::endpoints::elastic;
use crate::errors::WebResponse;
use crate::wrappers::DocumentJson;
use crate::context::SearchContext;

use actix_web::{delete, get, post, put, web};
use actix_web::{HttpResponse, ResponseError};
use elasticsearch::CreateParts;
use serde_json::json;


#[post("/{document}")]
async fn create_index(
    cxt: web::Data<SearchContext>,
    path: web::Path<String>,
) -> WebResponse<web::Json<DocumentJson>> {
    let elastic = cxt.get_cxt().lock().unwrap();
    let response = elastic
        .create(CreateParts::IndexId("tweets", "1"))
        .body(json!({
            "id": 1,
            "user": "kimchy",
            "post_date": "2009-11-15T00:00:00Z",
            "message": "Trying out Elasticsearch, so far so good?",
        }))
        .send()
        .await
        .unwrap();

    let result = response.status_code().is_success();
    Ok(web::Json(DocumentJson::new("tweets", "/tmp", ".txt")))
}

#[get("/{document}")]
async fn find_index(
    cxt: web::Data<SearchContext>,
    path: web::Path<String>,
) -> WebResponse<web::Json<DocumentJson>> {
    let elastic = cxt.get_cxt().lock().unwrap();
    let response = elastic
        .create(CreateParts::IndexId("tweets", "1"))
        .body(json!({
            "id": 1,
            "user": "kimchy",
            "post_date": "2009-11-15T00:00:00Z",
            "message": "Trying out Elasticsearch, so far so good?",
        }))
        .send()
        .await
        .unwrap();

    let result = response.status_code().is_success();
    Ok(web::Json(DocumentJson::new("tweets", "/tmp", ".txt")))
}
