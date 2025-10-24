use mockall::mock;

use crate::application::services::usermanager::UserManager;
use crate::application::services::usermanager::UserManagerError;
use crate::application::structures::Resource;

mock! {
    pub UserManagerClient{}

    #[async_trait::async_trait]
    impl UserManager for UserManagerClient {
        async fn get_user_resource(&self, user_id: &str) -> Result<Vec<Resource>, UserManagerError>;
        async fn check_user_access(&self, user_id: &str, resource: &str) -> Result<bool, UserManagerError>;
    }
}
