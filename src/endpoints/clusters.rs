use crate::context::SearchContext;
use crate::errors::{SuccessfulResponse, WebError, WebResponse};
use crate::wrappers::{Cluster, ClusterForm};

use actix_web::{delete, get, post, web, HttpResponse, ResponseError};
use elasticsearch::http::{headers::HeaderMap, Method};
use serde_json::{json, Value};

#[get("/clusters")]
async fn all_clusters(cxt: web::Data<SearchContext>) -> WebResponse<web::Json<Vec<Cluster>>> {
    let elastic = cxt.get_cxt().read().await;
    let response_result = elastic
        .send(
            Method::Get,
            "/_cat/nodes",
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(b"".as_ref()),
            None,
        )
        .await;

    if response_result.is_err() {
        let err = response_result.err().unwrap();
        return Err(WebError::from(err));
    }

    let response = response_result.unwrap();
    match response.json::<Vec<Cluster>>().await {
        Ok(json_clusters) => Ok(web::Json(json_clusters)),
        Err(err) => Err(WebError::GetClusterError(err.to_string())),
    }
}

#[post("/cluster/new")]
async fn new_cluster(cxt: web::Data<SearchContext>, form: web::Json<ClusterForm>) -> HttpResponse {
    let _elastic = cxt.get_cxt().read().await;
    let _cluster_name = form.0;

    // TODO: Add executing command from cli.
    let msg = "This method is not implemented yet";
    let web_err = WebError::CreateCluster(msg.to_string());
    web_err.error_response()
}

#[delete("/cluster/{cluster_name}")]
async fn delete_cluster(cxt: web::Data<SearchContext>, path: web::Path<String>) -> HttpResponse {
    let elastic = cxt.get_cxt().read().await;
    let cluster_name = path.to_string();
    let json_data: Value = json!({
        "transient": {
            "cluster.routing.allocation.exclude._ip": cluster_name
        }
    });

    let body = json_data.as_str();
    if body.is_none() {
        let msg = String::from("Json body is None");
        let web_err = WebError::DeletingCluster(msg);
        return web_err.error_response();
    }

    let body = body.unwrap().as_bytes();
    let response_result = elastic
        .send(
            Method::Put,
            "/_cluster/settings",
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body),
            None,
        )
        .await;

    match response_result {
        Ok(_) => SuccessfulResponse::ok_response("Ok"),
        Err(err) => {
            let web_err = WebError::DeletingClusterError(err.to_string());
            web_err.error_response()
        }
    }
}

#[get("/cluster/{cluster_name}")]
async fn get_cluster(
    cxt: web::Data<SearchContext>,
    path: web::Path<String>,
) -> WebResponse<web::Json<Value>> {
    let elastic = cxt.get_cxt().read().await;
    let cluster_name = format!("/_nodes/{}", path.to_string());
    let body = b"";
    let response_result = elastic
        .send(
            Method::Get,
            cluster_name.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body.as_ref()),
            None,
        )
        .await;

    if response_result.is_err() {
        let err = response_result.err().unwrap();
        return Err(WebError::from(err));
    }

    let response = response_result.unwrap();
    match response.json::<Value>().await {
        Ok(cluster_info) => Ok(web::Json(cluster_info)),
        Err(err) => Err(WebError::GetClusterError(err.to_string())),
    }
}
