mod config;
mod dto;

pub use config::BgeConfig;

use std::sync::Arc;

use crate::application::services::tokenizer::{TokenizeError, TokenizeProvider, TokenizeResult};
use crate::application::structures::{InputContent, TokenizedContent};
use crate::infrastructure::bge::dto::{InputForm, ResultFormBuilder, TokensData};
use crate::ServiceConnect;

const NATIVE_SERVICE_URL: &str = "/embeddings";

#[derive(Clone)]
pub struct BgeClient {
    config: BgeConfig,
    client: Arc<reqwest::Client>,
}

#[async_trait::async_trait]
impl ServiceConnect for BgeClient {
    type Config = BgeConfig;
    type Error = reqwest::Error;
    type Client = Self;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        tracing::debug!(url=config.address(), "connected to embeddings service");
        Ok(BgeClient {
            config: config.clone(),
            client: Arc::new(reqwest::Client::new()),
        })
    }
}

#[async_trait::async_trait]
impl TokenizeProvider for BgeClient {
    async fn compute(&self, form: &InputContent) -> TokenizeResult<TokenizedContent> {
        let input_form: InputForm = form.into();
        let target_url = format!("{}{}", self.config.address(), NATIVE_SERVICE_URL);

        let response = self
            .client
            .clone()
            .post(target_url)
            .json(&input_form)
            .send()
            .await?;

        if !response.status().is_success() {
            let err = response
                .error_for_status()
                .err()
                .unwrap();
            return Err(TokenizeError::ServiceError(err));
        }

        let content = response.json::<Vec<TokensData>>().await?;
        let content = ResultFormBuilder::default()
            .text(form.content().clone())
            .data(content)
            .build()
            .unwrap();

        Ok(content.into())
    }
}
