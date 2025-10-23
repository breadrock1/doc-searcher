mod config;
pub use config::UserManagerConfig;

mod dto;

use anyhow::anyhow;
use std::sync::Arc;

use crate::application::services::usermanager::{UserManager, UserManagerError, UserManagerResult};
use crate::application::structures::Resource;
use crate::infrastructure::usermanager::dto::{GetUserResourcesForm, ResourceSchema};
use crate::ServiceConnect;

const RESOURCES_URL: &str = "/resources";

pub struct UserManagerClient {
    config: UserManagerConfig,
    client: Arc<reqwest::Client>,
}

#[async_trait::async_trait]
impl ServiceConnect for UserManagerClient {
    type Config = UserManagerConfig;
    type Client = UserManagerClient;
    type Error = UserManagerError;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        tracing::debug!(url = config.address(), "connected to user manager service");
        Ok(UserManagerClient {
            config: config.clone(),
            client: Arc::new(reqwest::Client::new()),
        })
    }
}

#[async_trait::async_trait]
impl UserManager for UserManagerClient {
    async fn get_user_resource(&self, user_id: &str) -> UserManagerResult<Vec<Resource>> {
        let form = GetUserResourcesForm::new(user_id.to_string());
        let target_url = format!("{}{}", self.config.address(), RESOURCES_URL);
        let response = self
            .client
            .clone()
            .post(target_url)
            .json(&form)
            .send()
            .await?;

        if !response.status().is_success() {
            let err = response.error_for_status().err().unwrap();
            return Err(UserManagerError::InternalError(anyhow!(err)));
        }

        let resources = response
            .json::<Vec<ResourceSchema>>()
            .await?
            .into_iter()
            .map(|it| it.into())
            .collect::<Vec<Resource>>();

        Ok(resources)
    }

    async fn check_user_access(&self, _user_id: &str, _resource: &str) -> UserManagerResult<bool> {
        unimplemented!()
    }
}
