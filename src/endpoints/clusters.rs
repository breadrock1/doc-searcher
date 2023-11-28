use crate::endpoints::ContextData;
use crate::errors::WebResponse;
use crate::wrappers::cluster::{Cluster, ClusterForm};

use actix_web::{delete, get, post, web, HttpResponse};

#[get("/clusters")]
async fn all_clusters(cxt: ContextData) -> WebResponse<web::Json<Vec<Cluster>>> {
    let client = cxt.get_ref();
    client.get_all_clusters().await
}

#[post("/cluster/new")]
async fn new_cluster(cxt: ContextData, form: web::Json<ClusterForm>) -> HttpResponse {
    let cluster_name = form.0.to_string();
    let client = cxt.get_ref();
    client.create_cluster(cluster_name.as_str()).await
}

#[delete("/cluster/{cluster_name}")]
async fn delete_cluster(cxt: ContextData, path: web::Path<String>) -> HttpResponse {
    let client = cxt.get_ref();
    let cluster_name = path.to_string();
    client.delete_cluster(cluster_name.as_str()).await
}

#[get("/cluster/{cluster_name}")]
async fn get_cluster(cxt: ContextData, path: web::Path<String>) -> WebResponse<web::Json<Cluster>> {
    let client = cxt.get_ref();
    let cluster_name = format!("/_nodes/{}", path);
    client.get_cluster(cluster_name.as_str()).await
}

#[cfg(test)]
mod cluster_endpoints {
    use crate::errors::SuccessfulResponse;
    use crate::searcher::elastic::build_elastic_client;
    use crate::searcher::elastic::context::ElasticContext;
    use crate::service::{build_service, init_service_parameters};
    use crate::wrappers::cluster::Cluster;

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

        let elastic = build_elastic_client(es_host, es_user, es_passwd).unwrap();
        let cxt = ElasticContext::_new(elastic);
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
