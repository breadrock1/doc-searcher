use crate::context::SearchContext;
use crate::errors::{SuccessfulResponse, WebError, WebResponse};
use crate::wrappers::cluster::{Cluster, ClusterForm};

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
        Err(err) => Err(WebError::GetCluster(err.to_string())),
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
            let web_err = WebError::DeletingCluster(err.to_string());
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
        Err(err) => Err(WebError::GetCluster(err.to_string())),
    }
}

#[cfg(test)]
mod cluster_endpoints {
    use crate::context::SearchContext;
    use crate::errors::SuccessfulResponse;
    use crate::es_client::{build_elastic, build_service, init_service_parameters};
    use crate::wrappers::Cluster;

    use actix_web::test::TestRequest;
    use actix_web::{test, web, App};
    use serde_json::json;

    #[test]
    async fn build_application() {
        let service_parameters = init_service_parameters().unwrap();
        let es_host = service_parameters.es_host();
        let es_user = service_parameters.es_user();
        let es_passwd = service_parameters.es_passwd();
        let service_port = service_parameters.service_port();
        let service_addr = service_parameters.service_address();

        let elastic = build_elastic(es_host, es_user, es_passwd).unwrap();
        let cxt = SearchContext::_new(elastic);
        let app = App::new()
            .app_data(web::Data::new(cxt))
            .service(build_service());

        let test_app = test::init_service(app).await;
        let test_cluster_name = "test_cluster";

        // Create new cluster with name: "test_cluster"
        let create_cluster_resp = TestRequest::post()
            .uri("/searcher/cluster/new")
            .set_json(&json!({"cluster_name": test_cluster_name}))
            .send_request(&test_app)
            .await;

        let new_cluster: SuccessfulResponse = test::read_body_json(create_cluster_resp).await;
        assert_eq!(new_cluster.code, 400);

        // Get all clusters request
        let get_all_clusters_resp = TestRequest::get()
            .uri("/searcher/clusters")
            .send_request(&test_app)
            .await;

        let get_all_clusters: Vec<Cluster> = test::read_body_json(get_all_clusters_resp).await;
        assert_eq!(get_all_clusters.len() > 0, true);

        // Get cluster request by index
        // let get_cluster_resp = TestRequest::get()
        //     .uri(&format!("/searcher/cluster/{}", test_cluster_name))
        //     .send_request(&test_app)
        //     .await;
        //
        // let get_cluster: Cluster = test::read_body_json(get_cluster_resp).await;
        // assert_eq!(get_cluster.name(), test_cluster_name);

        // TODO: Skip deleting cluster test
        // Delete cluster by index
        // let delete_cluster_resp = TestRequest::delete()
        //     .uri(&format!("/searcher/cluster/{}", test_cluster_name))
        //     .send_request(&test_app)
        //     .await;
        //
        // let delete_cluster: SuccessfulResponse = test::read_body_json(delete_cluster_resp).await;
        // assert_eq!(delete_cluster.code, 200);

        // TODO: Skip getting cluster test
        // Get cluster by index -> get error message
        // let delete_cluster_resp = TestRequest::get()
        //     .uri(&format!("/searcher/cluster/{}", "lsdfnbsikdjfsidg"))
        //     .send_request(&test_app)
        //     .await;
        //
        // let delete_cluster: ErrorResponse = test::read_body_json(delete_cluster_resp).await;
        // assert_eq!(delete_cluster.code, 400);
    }
}
