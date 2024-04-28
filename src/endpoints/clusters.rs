use crate::endpoints::SearcherData;
use crate::errors::{ErrorResponse, JsonResponse, SuccessfulResponse};

use actix_web::{delete, get, post, web, HttpResponse};

use wrappers::cluster::{Cluster, ClusterForm};

#[utoipa::path(
    get,
    path = "/cluster/all",
    tag = "Clusters",
    responses(
        (
            status = 200,
            description = "Successful",
            body = [Cluster],
            example = json!(vec![Cluster::default()])
        ),
        (
            status = 400,
            description = "Failed while getting clusters",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting clusters".to_string(),
            })
        ),
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
    tag = "Clusters",
    request_body(
        content = ClusterForm,
        example = json!({
            "cluster_name": "test_slave"
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
            description = "Failed while creating cluster",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while creating cluster".to_string(),
            })
        ),
        (
            status = 501,
            description = "Failed while creating cluster",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 501,
                error: "Not Implemented".to_string(),
                message: "Not implemented functionality yet".to_string(),
            })
        ),
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
    tag = "Clusters",
    params(
        (
            "cluster_name" = &str, 
            description = "Cluster name to delete",
            example = "d93df49fa6ft",
        )
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
            description = "Failed while deleting cluster",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while deleting cluster".to_string(),
            })
        ),
        (
            status = 501,
            description = "Failed while deleting cluster",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 501,
                error: "Not Implemented".to_string(),
                message: "Not implemented functionality yet".to_string(),
            })
        ),
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
    tag = "Clusters",
    params(
        (
            "cluster_name" = &str, 
            description = "Cluster name to get",
            example = "d93df49fa6ff",
        )
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Cluster,
            example = json!(Cluster::default())
        ),
        (
            status = 400,
            description = "Failed while getting cluster by name",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting cluster by name".to_string(),
            })
        ),
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
