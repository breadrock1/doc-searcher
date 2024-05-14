use crate::errors::{ErrorResponse, JsonResponse, SuccessfulResponse};
use crate::services::searcher::ClustersService;

use crate::forms::cluster::{Cluster, ClusterForm};
use crate::forms::TestExample;

use actix_web::{delete, get, post, web, HttpResponse, ResponseError};

type Context = web::Data<Box<dyn ClustersService>>;

#[utoipa::path(
    get,
    path = "/clusters/",
    tag = "Clusters",
    responses(
        (
            status = 200,
            description = "Successful",
            body = [Cluster],
            example = json!(vec![Cluster::test_example(None)])
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
#[get("/")]
async fn all_clusters(cxt: Context) -> JsonResponse<Vec<Cluster>> {
    let client = cxt.get_ref();
    client.get_all_clusters().await
}

#[utoipa::path(
    post,
    path = "/clusters/create",
    tag = "Clusters",
    request_body(
        content = ClusterForm,
        example = json!({
            "cluster_id": "test_slave"
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
#[post("/create")]
async fn create_cluster(cxt: Context, form: web::Json<ClusterForm>) -> HttpResponse {
    let cluster_id = form.0.to_string();
    let client = cxt.get_ref();
    match client.create_cluster(cluster_id.as_str()).await {
        Ok(response) => response.to_response(),
        Err(err) => err.error_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/clusters/{cluster_id}",
    tag = "Clusters",
    params(
        (
            "cluster_id" = &str,
            description = "Cluster id to delete",
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
#[delete("/{cluster_id}")]
async fn delete_cluster(cxt: Context, path: web::Path<String>) -> HttpResponse {
    let client = cxt.get_ref();
    match client.delete_cluster(path.as_str()).await {
        Ok(response) => response.to_response(),
        Err(err) => err.error_response(),
    }
}

#[utoipa::path(
    get,
    path = "/clusters/{cluster_id}",
    tag = "Clusters",
    params(
        (
            "cluster_id" = &str,
            description = "Cluster id to get",
            example = "d93df49fa6ff",
        )
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Cluster,
            example = json!(Cluster::test_example(None))
        ),
        (
            status = 400,
            description = "Failed while getting cluster by id",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting cluster by id".to_string(),
            })
        ),
    )
)]
#[get("/{cluster_id}")]
async fn get_cluster(cxt: Context, path: web::Path<String>) -> JsonResponse<Cluster> {
    let client = cxt.get_ref();
    client.get_cluster(path.as_str()).await
}

#[cfg(test)]
mod cluster_endpoints {
    use crate::services::own_engine::context::OtherContext;
    use crate::services::searcher::ClustersService;

    use actix_web::test;

    #[test]
    async fn create_cluster() {
        let other_context = OtherContext::new("test".to_string());
        let response = other_context.create_cluster("test_cluster").await;
        assert_eq!(response.unwrap().code, 200_u16);
    }

    #[test]
    async fn delete_cluster() {
        let other_context = OtherContext::new("test".to_string());
        let _ = other_context.create_cluster("test_cluster").await;
        let response = other_context.delete_cluster("test_cluster").await;
        assert_eq!(response.unwrap().code, 200_u16);
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
        assert_eq!(response.unwrap().get_ip(), "localhost");
    }
}
