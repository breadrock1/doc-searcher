use crate::endpoints::document;
use crate::errors::{WebError, WebResponse};
use crate::wrappers::{Cluster, ClusterForm, ClusterResult, DocumentJson};
use crate::context::SearchContext;

use actix_web::{delete, get, post, put, web};
use actix_web::{HttpResponse, ResponseError};
use elasticsearch::{CreateParts, Error, SearchParts};
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::Method;
use serde_json::{json, Value};
use log::{info, error};

#[post("/cluster/new")]
async fn new_cluster(
    cxt: web::Data<SearchContext>,
    form: web::Form<ClusterForm>,
) -> WebResponse<web::Json<ClusterResult>> {
    let elastic = cxt.get_cxt().lock().unwrap();
    let cluster_name = form.0;

    let body = b"";
    let response = elastic
        .send(
            Method::Get,
            "/_cat/nodes",
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body.as_ref()),
            None,
        )
        .await
        .unwrap();

    // let result = response.status_code().as_u16();
    // Ok(web::Json(ClusterResult::new(result)))

    let result = response.text().await.unwrap();;
    println!("{:?}", result.clone());

    Ok(web::Json(ClusterResult::new(200u16)))
}

#[get("/cluster/all")]
async fn all_clusters(
    cxt: web::Data<SearchContext>
) -> WebResponse<web::Json<Vec<Cluster>>> {
    let elastic = cxt.get_cxt().lock().unwrap();

    let body = b"";
    let response = elastic
        .send(
            Method::Get,
            "/_cat/nodes",
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body.as_ref()),
            None,
        )
        .await
        .unwrap();

    match response.json::<Vec<Cluster>>().await {
        Ok(json_clusters) => Ok(web::Json(json_clusters)),
        Err(err) => {
            error!("{:?}", err);
            Err(WebError::SomeError(err.to_string()))
        }
    }
}
