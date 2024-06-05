use crate::errors::{Successful, WebError, WebResult};
use crate::forms::clusters::cluster::Cluster;
use crate::forms::clusters::forms::CreateClusterForm;
use crate::services::searcher::elastic::context::ElasticContext;
use crate::services::searcher::elastic::helper;
use crate::services::searcher::service::ClusterService;

use elasticsearch::http::Method;
use serde_json::{json, Value};

#[async_trait::async_trait]
impl ClusterService for ElasticContext {
    async fn get_all_clusters(&self) -> WebResult<Vec<Cluster>> {
        let elastic = self.get_cxt().read().await;
        let response = helper::send_elrequest(&elastic, Method::Get, None, "/_cat/nodes").await?;
        match response.json::<Vec<Cluster>>().await {
            Ok(clusters) => Ok(clusters),
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                Err(WebError::from(err))
            }
        }
    }
    async fn get_cluster(&self, cluster_id: &str) -> WebResult<Cluster> {
        let elastic = self.get_cxt().read().await;
        let response = helper::send_elrequest(&elastic, Method::Get, None, "/_cat/nodes").await?;
        let clusters = response
            .json::<Vec<Cluster>>()
            .await
            .map_err(WebError::from)?;

        let founded_cluster = clusters
            .iter()
            .filter(|cluster| cluster.get_name().eq(cluster_id))
            .map(|cluster| cluster.to_owned())
            .collect::<Vec<Cluster>>();

        match founded_cluster.first() {
            Some(value) => Ok(value.to_owned()),
            None => {
                log::warn!("There is no cluster with passed name: {}", cluster_id);
                Err(WebError::GetCluster(cluster_id.to_string()))
            }
        }
    }
    async fn delete_cluster(&self, cluster_id: &str) -> WebResult<Successful> {
        let elastic = self.get_cxt().read().await;
        let json_data: Value = json!({
            "transient": {
                "cluster.routing.allocation.exclude._ip": cluster_id
            }
        });

        let json_str = serde_json::to_string(&json_data).unwrap();
        let target_url = "/_cluster/settings";
        let body = json_str.as_bytes();
        let response =
            helper::send_elrequest(&elastic, Method::Put, Some(body), target_url).await?;
        helper::parse_elastic_response(response).await
    }
    async fn create_cluster(&self, _id: &str, _form: &CreateClusterForm) -> WebResult<Successful> {
        log::warn!("This functionality does not implemented yet!");
        Err(WebError::CreateCluster("Not available".to_string()))
    }
}
