use crate::errors::{JsonResponse, SuccessfulResponse, WebError};
use crate::forms::cluster::Cluster;
use crate::services::elastic::{context, helper};
use crate::services::searcher;

use actix_web::web;
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::Method;
use serde_json::{json, Value};

#[async_trait::async_trait]
impl searcher::ClustersService for context::ElasticContext {
    async fn get_all_clusters(&self) -> JsonResponse<Vec<Cluster>> {
        let elastic = self.get_cxt().read().await;
        let response = helper::get_all_clusters(&elastic).await?;
        match response.json::<Vec<Cluster>>().await {
            Ok(clusters) => Ok(web::Json(clusters)),
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                Err(WebError::from(err))
            }
        }
    }
    async fn get_cluster(&self, cluster_id: &str) -> JsonResponse<Cluster> {
        let elastic = self.get_cxt().read().await;
        let response = helper::get_all_clusters(&elastic).await?;
        match response.json::<Vec<Cluster>>().await {
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                Err(WebError::from(err))
            }
            Ok(clusters) => {
                let founded_cluster = clusters
                    .iter()
                    .filter(|cluster| cluster.get_name().eq(cluster_id))
                    .map(|cluster| cluster.to_owned())
                    .collect::<Vec<Cluster>>();

                match founded_cluster.first() {
                    Some(value) => Ok(web::Json(value.to_owned())),
                    None => {
                        let msg = format!("There is no cluster with passed name: {}", cluster_id);
                        log::error!("{}", msg.as_str());
                        Err(WebError::GetCluster(msg))
                    }
                }
            }
        }
    }
    async fn create_cluster(&self, _cluster_id: &str) -> Result<SuccessfulResponse, WebError> {
        let msg = "This functionality does not implemented yet!";
        log::warn!("{}", msg);
        Err(WebError::CreateCluster(msg.to_string()))
    }
    async fn delete_cluster(&self, cluster_id: &str) -> Result<SuccessfulResponse, WebError> {
        let elastic = self.get_cxt().read().await;
        let json_data: Value = json!({
            "transient": {
                "cluster.routing.allocation.exclude._ip": cluster_id
            }
        });

        let json_str = serde_json::to_string(&json_data).unwrap();
        let body = json_str.as_bytes();
        let response = elastic
            .send(
                Method::Put,
                "/_cluster/settings",
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(body),
                None,
            )
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }
}
