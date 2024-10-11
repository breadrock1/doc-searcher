use crate::orchestra::forms::CreateClusterForm;
use crate::orchestra::models::Cluster;
use crate::orchestra::ClusterService;
use crate::errors::{Successful, WebError, WebErrorEntity, WebResult};
use crate::searcher::elastic::helper;
use crate::elastic::ElasticClient;

use elasticsearch::http::Method;
use serde_json::{json, Value};

const CAT_NODES_URL: &str = "/_cat/nodes";
const CLUSTER_SETTINGS_URL: &str = "/_cluster/settings";

#[async_trait::async_trait]
impl ClusterService for ElasticClient {
    async fn get_all_clusters(&self) -> WebResult<Vec<Cluster>> {
        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let response = helper::send_elrequest(&elastic, Method::Get, None, CAT_NODES_URL).await?;
        response.json::<Vec<Cluster>>().await.map_err(WebError::from)
    }

    async fn get_cluster(&self, cluster_id: &str) -> WebResult<Cluster> {
        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let response = helper::send_elrequest(&elastic, Method::Get, None, CAT_NODES_URL).await?;
        let clusters = response
            .json::<Vec<Cluster>>()
            .await
            .map_err(WebError::from)?;

        let founded_cluster = clusters
            .iter()
            .filter(|cluster| cluster.name().eq(cluster_id))
            .map(|cluster| cluster.to_owned())
            .collect::<Vec<Cluster>>();

        match founded_cluster.first() {
            Some(value) => Ok(value.to_owned()),
            None => {
                tracing::warn!("There is no cluster with passed name: {cluster_id}");
                let entity = WebErrorEntity::new(cluster_id.to_string());
                Err(WebError::GetCluster(entity))
            }
        }
    }

    async fn delete_cluster(&self, cluster_id: &str) -> WebResult<Successful> {
        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let json_data: Value = json!({
            "transient": {
                "cluster.routing.allocation.exclude._ip": cluster_id
            }
        });

        let json_str = serde_json::to_string(&json_data).unwrap();
        let body = json_str.as_bytes();
        let response = helper::send_elrequest(
            &elastic,
            Method::Put,
            Some(body),
            CLUSTER_SETTINGS_URL,
        ).await?;

        helper::parse_elastic_response(response).await
    }

    async fn create_cluster(&self, _id: &str, _form: &CreateClusterForm) -> WebResult<Successful> {
        tracing::warn!("This functionality does not implemented yet!");
        let entity = WebErrorEntity::new("Not available".to_string());
        Err(WebError::CreateCluster(entity))
    }
}
