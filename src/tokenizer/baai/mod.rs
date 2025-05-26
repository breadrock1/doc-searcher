pub mod config;
mod model;

use std::sync::Arc;

use crate::tokenizer::baai::config::BAAIConfig;
use crate::tokenizer::baai::model::BAAIForm;
use crate::tokenizer::errors::{TokenizerError, TokenizerResult};
use crate::tokenizer::TokenizerService;
use crate::ServiceConnect;

const NATIVE_SERVICE_URL: &str = "/embed";

#[derive(Clone)]
pub struct BAAIClient {
    config: BAAIConfig,
    client: Arc<reqwest::Client>,
}

#[async_trait::async_trait]
impl ServiceConnect for BAAIClient {
    type Config = BAAIConfig;
    type Error = reqwest::Error;
    type Client = Self;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        tracing::info!(address=config.address(), "connected to BAAI OCR service");
        Ok(BAAIClient {
            config: config.clone(),
            client: Arc::new(reqwest::Client::new()),
        })
    }
}

#[async_trait::async_trait]
impl TokenizerService for BAAIClient {
    async fn compute(&self, text: &str) -> TokenizerResult<Vec<f64>> {
        let target_url = format!("{}{}", self.config.address(), NATIVE_SERVICE_URL);
        let form = BAAIForm::builder()
            .inputs(text.to_owned())
            .normalize(self.config.is_normalize())
            .truncate(self.config.is_truncate())
            .build()
            .map_err(|err| TokenizerError::ConnectError(err.to_string()))?;

        let response = self
            .client
            .clone()
            .post(target_url)
            .json(&form)
            .send()
            .await?;

        let embed_data = response.json::<Vec<Vec<f64>>>().await?;
        match embed_data.first() {
            Some(tokens) => Ok(tokens.to_owned()),
            None => Err(TokenizerError::EmptyResponse),
        }
    }
}
