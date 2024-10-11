pub mod elastic;
pub mod endpoints;
pub mod forms;
pub mod models;

use crate::orchestra::forms::CreateClusterForm;
use crate::orchestra::models::Cluster;
use crate::errors::{Successful, WebResult};

#[async_trait::async_trait]
pub trait ClusterService {
    async fn get_all_clusters(&self) -> WebResult<Vec<Cluster>>;
    async fn get_cluster(&self, id: &str) -> WebResult<Cluster>;
    async fn delete_cluster(&self, id: &str) -> WebResult<Successful>;
    async fn create_cluster(&self, id: &str, form: &CreateClusterForm) -> WebResult<Successful>;
}
