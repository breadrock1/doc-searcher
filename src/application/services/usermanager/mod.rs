mod error;
pub use error::{UserManagerError, UserManagerResult};

use crate::application::structures::Resource;

#[async_trait::async_trait]
pub trait UserManager {
    async fn get_user_resource(&self, user_id: &str) -> UserManagerResult<Vec<Resource>>;
    async fn check_user_access(&self, user_id: &str, resource: &str) -> UserManagerResult<bool>;
}
