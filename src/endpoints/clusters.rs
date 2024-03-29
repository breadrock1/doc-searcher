use crate::endpoints::SearcherData;
use crate::errors::JsonResponse;

use wrappers::cluster::{Cluster, ClusterForm};

use actix_web::{delete, get, post, web, HttpResponse};

#[utoipa::path(
    get,
    path = "/cluster/all",
    tag = "Get all available clusters",
    responses(
        (status = 200, description = "Successful", body = [Cluster]),
        (status = 401, description = "Failed while getting clusters", body = ErrorResponse),
    )
)]
#[get("/all")]
async fn all_clusters(cxt: SearcherData) -> JsonResponse<Vec<Cluster>> {
    let client = cxt.get_ref();
    client.get_all_clusters().await
}

#[utoipa::path(
    post,
    path = "/cluster/new",
    tag = "Create new Cluster by ClusterForm",
    request_body = ClusterForm,
    responses(
        (status = 200, description = "Successful", body = SuccessfulResponse),
        (status = 401, description = "Failed while creating cluster", body = ErrorResponse),
    )
)]
#[post("/new")]
async fn new_cluster(cxt: SearcherData, form: web::Json<ClusterForm>) -> HttpResponse {
    let cluster_name = form.0.to_string();
    let client = cxt.get_ref();
    client.create_cluster(cluster_name.as_str()).await
}

#[utoipa::path(
    delete,
    path = "/cluster/{cluster_name}",
    tag = "Delete cluster by name",
    params(
        ("cluster_name" = &str, description = "Cluster name to delete")
    ),
    responses(
        (status = 200, description = "Successful", body = SuccessfulResponse),
        (status = 401, description = "Failed while deleting cluster", body = ErrorResponse),
    )
)]
#[delete("/{cluster_name}")]
async fn delete_cluster(cxt: SearcherData, path: web::Path<String>) -> HttpResponse {
    let client = cxt.get_ref();
    let cluster_name = path.to_string();
    client.delete_cluster(cluster_name.as_str()).await
}

#[utoipa::path(
    get,
    path = "/cluster/{cluster_name}",
    tag = "Getting cluster by name",
    params(
        ("cluster_name" = &str, description = "Cluster name to get")
    ),
    responses(
        (status = 200, description = "Successful", body = Cluster),
        (status = 401, description = "Failed while getting cluster", body = ErrorResponse),
    )
)]
#[get("/{cluster_name}")]
async fn get_cluster(cxt: SearcherData, path: web::Path<String>) -> JsonResponse<Cluster> {
    let client = cxt.get_ref();
    client.get_cluster(path.as_str()).await
}

#[cfg(test)]
mod cluster_endpoints {
    use crate::services::own_engine::context::OtherContext;
    use crate::services::SearcherService;

    use actix_web::test;

    #[test]
    async fn create_cluster() {
        let other_context = OtherContext::new("test".to_string());
        let response = other_context.create_cluster("test_cluster").await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }

    #[test]
    async fn delete_cluster() {
        let other_context = OtherContext::new("test".to_string());
        let _ = other_context.create_cluster("test_cluster").await;
        let response = other_context.delete_cluster("test_cluster").await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }

    #[test]
    async fn get_clusters() {
        let other_context = OtherContext::new("test".to_string());
        let _ = other_context.create_cluster("test_cluster").await;
        let response = other_context.get_all_clusters().await;
        assert_eq!(response.unwrap().len(), 1);
    }

    #[test]
    async fn get_cluster_by_id() {
        let other_context = OtherContext::new("test".to_string());
        let _ = other_context.create_cluster("test_cluster").await;
        let response = other_context.get_cluster("test_cluster").await;
        assert_eq!(response.unwrap().ip, "localhost");
    }
}
