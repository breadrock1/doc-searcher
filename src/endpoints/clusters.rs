use crate::errors::{ErrorResponse, JsonResponse, Successful};
use crate::forms::TestExample;
use crate::forms::clusters::cluster::Cluster;
use crate::forms::clusters::forms::CreateClusterForm;
use crate::services::searcher::service::ClusterService;

use actix_web::{delete, get, put};
use actix_web::web::{Data, Json, Path};

type Context = Data<Box<dyn ClusterService>>;

#[utoipa::path(
    get,
    path = "/orchestra/clusters",
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
                attachments: None,
            })
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 503,
                error: "Server error".to_string(),
                message: "Server does not available".to_string(),
                attachments: None,
            })
        )
    )
)]
#[get("/clusters")]
async fn get_clusters(cxt: Context) -> JsonResponse<Vec<Cluster>> {
    let client = cxt.get_ref();
    Ok(Json(client.get_all_clusters().await?))
}

#[utoipa::path(
    get,
    path = "/orchestra/clusters/{cluster_id}",
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
                attachments: None,
            })
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 503,
                error: "Server error".to_string(),
                message: "Server does not available".to_string(),
                attachments: None,
            })
        )
    )
)]
#[get("/clusters/{cluster_id}")]
async fn get_cluster(cxt: Context, path: Path<String>) -> JsonResponse<Cluster> {
    let client = cxt.get_ref();
    Ok(Json(client.get_cluster(path.as_str()).await?))
}

#[utoipa::path(
    put,
    path = "/orchestra/clusters/{cluster_id}",
    tag = "Clusters",
    request_body(
        content = CreateClusterForm,
        example = json!(CreateClusterForm::test_example(None))
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful {
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
                attachments: None,
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
                attachments: None,
            })
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 503,
                error: "Server error".to_string(),
                message: "Server does not available".to_string(),
                attachments: None,
            })
        )
    )
)]
#[put("/clusters/{cluster_id}")]
async fn create_cluster(
    cxt: Context, 
    path: Path<String>,
    form: Json<CreateClusterForm>,
) -> JsonResponse<Successful> {
    let cluster_form = form.0;
    let cluster_id = path.as_str();
    let client = cxt.get_ref();
    let status = client.create_cluster(cluster_id, &cluster_form).await?;
    Ok(Json(status))
}

#[utoipa::path(
    delete,
    path = "/orchestra/clusters/{cluster_id}",
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
            body = Successful,
            example = json!(Successful {
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
                attachments: None,
            })
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 503,
                error: "Server error".to_string(),
                message: "Server does not available".to_string(),
                attachments: None,
            })
        )
    )
)]
#[delete("/clusters/{cluster_id}")]
async fn delete_cluster(cxt: Context, path: Path<String>) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let status = client.delete_cluster(path.as_str()).await?;
    Ok(Json(status))
}
