use crate::context::SearchContext;
use crate::errors::{WebError, WebResponse};
use crate::wrappers::{Cluster, ClusterForm, StatusResult};

use actix_web::{delete, get, post, web};
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::Method;
use serde_json::{json, Value};

#[get("/clusters")]
async fn all_clusters(cxt: web::Data<SearchContext>) -> WebResponse<web::Json<Vec<Cluster>>> {
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
        .await?;

    match response.json::<Vec<Cluster>>().await {
        Ok(json_clusters) => Ok(web::Json(json_clusters)),
        Err(err) => {
            println!("{:?}", err);
            Err(WebError::SomeError(err.to_string()))
        }
    }
}

#[post("/cluster/new")]
async fn new_cluster(
    cxt: web::Data<SearchContext>,
    form: web::Form<ClusterForm>,
) -> WebResponse<web::Json<StatusResult>> {
    let _elastic = cxt.get_cxt().lock().unwrap();
    let _cluster_name = form.0;

    // TODO: Add executing command from cli.

    let result = 200u16;
    Ok(web::Json(StatusResult::new(result)))
}

#[delete("/cluster/delete")]
async fn delete_cluster(
    cxt: web::Data<SearchContext>,
    form: web::Form<ClusterForm>,
) -> WebResponse<web::Json<StatusResult>> {
    let elastic = cxt.get_cxt().lock().unwrap();
    let cluster_name = form.0;

    let json_data: Value = json!({
        "transient": {
            "cluster.routing.allocation.exclude._ip": cluster_name
        }
    });

    let body = json_data.as_str().unwrap().as_bytes();
    let response = elastic
        .send(
            Method::Put,
            "/_cluster/settings",
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body.as_ref()),
            None,
        )
        .await?;

    let result = response.status_code().as_u16();
    Ok(web::Json(StatusResult::new(result)))
}

#[get("/cluster/{cluster_name}")]
async fn get_cluster(
    cxt: web::Data<SearchContext>,
    path: web::Path<String>,
) -> WebResponse<web::Json<Value>> {
    let elastic = cxt.get_cxt().lock().unwrap();

    let cluster_name = format!("/_nodes/{}", path.to_string());
    let body = b"";
    let response = elastic
        .send(
            Method::Get,
            cluster_name.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body.as_ref()),
            None,
        )
        .await?;

    match response.json::<Value>().await {
        Ok(cluster_info) => Ok(web::Json(cluster_info)),
        Err(err) => {
            println!("{:?}", err);
            Err(WebError::SomeError(err.to_string()))
        }
    }
}
