pub mod cacher;
pub mod values;

use crate::values::{CacherDocument, CacherSearchParams};

// #[async_trait::async_trait]
pub trait AnyCacherService {
    async fn get_documents(&self, params: &CacherSearchParams) -> Option<Vec<CacherDocument>>;
    async fn set_documents(&self, params: &CacherSearchParams, docs: Vec<CacherDocument>);
}
