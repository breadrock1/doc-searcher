use crate::errors::{JsonResponse, SuccessfulResponse, WebError, WebResult};
use crate::forms::cluster::Cluster;
use crate::services::own_engine::context::OtherContext;
use crate::services::service;

use actix_web::web;

#[async_trait::async_trait]
impl service::ClustersService for OtherContext {
    async fn get_all_clusters(&self) -> JsonResponse<Vec<Cluster>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.clusters.read().await;
        let clusters_vec = map.values().cloned().collect::<Vec<Cluster>>();
        Ok(web::Json(clusters_vec))
    }

    async fn get_cluster(&self, cluster_id: &str) -> JsonResponse<Cluster> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.clusters.read().await;
        match map.get(cluster_id) {
            Some(cluster) => Ok(web::Json(cluster.clone())),
            None => {
                log::warn!("Failed while getting cluster: {}", cluster_id);
                let msg = "failed to get cluster".to_string();
                Err(WebError::GetCluster(msg))
            }
        }
    }

    async fn create_cluster(&self, cluster_id: &str) -> WebResult {
        let cluster = Cluster::builder()
            .ip("localhost".to_string())
            .heap_percent("70%".to_string())
            .ram_percent("70%".to_string())
            .cpu("7/10".to_string())
            .load_1m("anh value".to_string())
            .load_5m("anh value".to_string())
            .load_15m("anh value".to_string())
            .master("master".to_string())
            .name(cluster_id.to_string())
            .node_role(cluster_id.to_string())
            .build()
            .unwrap();

        let cxt = self.get_cxt().write().await;
        let mut map = cxt.clusters.write().await;
        match map.insert(cluster_id.to_string(), cluster) {
            Some(_) => Ok(SuccessfulResponse::success("Updated")),
            None => {
                let msg = format!("Created {}", cluster_id);
                log::info!("New cluster has been created: {}", msg);
                Ok(SuccessfulResponse::success(msg.as_str()))
            }
        }
    }

    async fn delete_cluster(&self, cluster_id: &str) -> WebResult {
        let cxt = self.get_cxt().write().await;
        let mut map = cxt.clusters.write().await;
        match map.remove(cluster_id) {
            Some(_) => Ok(SuccessfulResponse::success("Ok")),
            None => {
                let msg = "Not exist cluster".to_string();
                log::warn!("Failed while deleting cluster: {}", msg.as_str());
                Err(WebError::DeleteCluster(msg))
            }
        }
    }
}

#[cfg(test)]
mod test_clusters {
    use crate::services::own_engine::context::OtherContext;
    use crate::services::service::ClustersService;

    use actix_web::test;

    const CLUSTER_ID: &str = "test_cluster";

    #[test]
    async fn create_cluster() {
        let other_context = OtherContext::new("test".to_string());
        let response = other_context.create_cluster(CLUSTER_ID).await;
        assert_eq!(response.unwrap().code, 200_u16);
    }

    #[test]
    async fn delete_cluster() {
        let other_context = OtherContext::new("test".to_string());
        let _ = other_context.create_cluster(CLUSTER_ID).await;
        let response = other_context.delete_cluster(CLUSTER_ID).await;
        assert_eq!(response.unwrap().code, 200_u16);
    }

    #[test]
    async fn get_clusters() {
        let other_context = OtherContext::new("test".to_string());
        let _ = other_context.create_cluster(CLUSTER_ID).await;
        let response = other_context.get_all_clusters().await;
        assert_eq!(response.unwrap().len(), 1);
    }

    #[test]
    async fn get_cluster_by_id() {
        let other_context = OtherContext::new("test".to_string());
        let _ = other_context.create_cluster(CLUSTER_ID).await;
        let response = other_context.get_cluster(CLUSTER_ID).await;
        assert_eq!(response.unwrap().get_ip(), "localhost");
    }
}
