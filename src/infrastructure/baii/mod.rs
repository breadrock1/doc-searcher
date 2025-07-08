pub mod config;
mod dto;

use std::sync::Arc;

use self::config::BAAIConfig;
use self::dto::VectorizerForm;

use crate::application::dto::Tokens;
use crate::application::services::tokenizer::{Tokenizer, TokenizerError, TokenizerResult};
use crate::ServiceConnect;

const NATIVE_SERVICE_URL: &str = "/embed";

#[derive(Clone)]
pub struct VectorizerClient {
    config: BAAIConfig,
    client: Arc<reqwest::Client>,
}

#[async_trait::async_trait]
impl ServiceConnect for VectorizerClient {
    type Config = BAAIConfig;
    type Error = reqwest::Error;
    type Client = Self;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        tracing::info!(
            address = config.address(),
            "connected to Vectorizer OCR service"
        );
        Ok(VectorizerClient {
            config: config.clone(),
            client: Arc::new(reqwest::Client::new()),
        })
    }
}

#[async_trait::async_trait]
impl Tokenizer for VectorizerClient {
    async fn compute(&self, text: &str) -> TokenizerResult<Tokens> {
        let target_url = format!("{}{}", self.config.address(), NATIVE_SERVICE_URL);
        let form = VectorizerForm::builder()
            .inputs(text.to_string())
            .normalize(self.config.is_normalize())
            .truncate(self.config.is_truncate())
            .build()
            .map_err(|err| TokenizerError::FormDataError(err.to_string()))?;

        let response = self
            .client
            .clone()
            .post(target_url)
            .json(&form)
            .send()
            .await?;

        let embed_data = response.json::<Vec<Vec<f64>>>().await?;
        if embed_data.is_empty() {
            return Err(TokenizerError::EmptyResponse);
        }

        let Some(embed) = embed_data.first() else {
            return Err(TokenizerError::EmptyResponse);
        };

        let tokens = Tokens::builder()
            .tokens(embed.to_owned())
            .build()
            .map_err(|err| TokenizerError::RuntimeError(err.to_string()))?;

        Ok(tokens)
    }
}
