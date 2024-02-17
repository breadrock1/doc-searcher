pub mod cacher;
pub mod values;

use crate::values::VecCacherDocuments;

use wrappers::search_params::*;

#[async_trait::async_trait]
pub trait AnyCacherService {
    async fn get_documents(&self, params: &SearchParams) -> Option<VecCacherDocuments>;
    async fn set_documents(&self, params: &SearchParams, docs: VecCacherDocuments) -> VecCacherDocuments;
}
