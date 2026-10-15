#[cfg(test)]
mod tests;

mod config;
pub use config::EmbeddingsConfig;

mod dto;

use metrics::{counter, histogram};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio_retry::RetryIf;
use tokio_retry::strategy::{ExponentialBackoff, jitter};
use tracing::instrument;

use crate::ServiceConnect;
use crate::application::services::embeddings::{ITokenizer, TokenizerError, TokenizerResult};
use crate::infrastructure::embeddings::dto::{EmbeddingsRequest, EmbeddingsResponse};
use crate::shared::kernel::Tokens;

pub const EMBEDDINGS_PATH: &str = "/embeddings";
const EXPONENTIAL_MIN_MILLS: u64 = 50;
const EXPONENTIAL_MAX_DELAY_SECS: u64 = 2;

#[derive(Clone)]
pub struct EmbeddingsClient {
    config: EmbeddingsConfig,
    client: Arc<reqwest::Client>,
}

#[async_trait::async_trait]
impl ServiceConnect for EmbeddingsClient {
    type Config = EmbeddingsConfig;
    type Client = EmbeddingsClient;
    type Error = reqwest::Error;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout()))
            .build()?;

        tracing::debug!(address = %config.address(), "connected to embeddings service");

        Ok(EmbeddingsClient {
            config: config.clone(),
            client: Arc::new(client),
        })
    }
}

#[async_trait::async_trait]
impl ITokenizer for EmbeddingsClient {
    #[instrument(level = "info", skip(self, text), fields(text_len = text.len()))]
    async fn compute(&self, text: &str) -> TokenizerResult<Tokens> {
        let instant = Instant::now();

        let (is_success, result) = match self.compute_with_retries(text).await {
            Ok(embedding) => ("true", Ok(embedding)),
            Err(err) => {
                tracing::error!(
                    text_len = text.len(),
                    ?err,
                    "embeddings server connection refused",
                );

                ("false", Ok(Tokens::default()))
            }
        };

        counter!("docsearch_embeddings_requests_total", "status" => is_success).increment(1);
        histogram!("docsearch_embeddings_request_duration_seconds")
            .record(instant.elapsed().as_secs_f64());

        result
    }
}

impl EmbeddingsClient {
    async fn compute_with_retries(&self, text: &str) -> TokenizerResult<Tokens> {
        let strategy = ExponentialBackoff::from_millis(EXPONENTIAL_MIN_MILLS)
            .max_delay(Duration::from_secs(EXPONENTIAL_MAX_DELAY_SECS))
            .map(jitter)
            .take(self.config.retries() as usize);

        let url = format!("{}{}", self.config.address(), EMBEDDINGS_PATH);
        let send = || async {
            let req_body = EmbeddingsRequest {
                text: text.to_owned(),
                model: self.config.model().clone(),
                overlap: self.config.overlap(),
            };

            let response = self
                .client
                .post(&url)
                .json(&req_body)
                .send()
                .await
                .map_err(|err| TokenizerError::Request(err.to_string()))?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                return Err(TokenizerError::UnexpectedStatus {
                    status: status.as_u16(),
                    body,
                });
            }

            let payload: EmbeddingsResponse = response
                .json()
                .await
                .map_err(|err| TokenizerError::InvalidResponse(err.to_string()))?;

            if payload.embedding.len() != self.config.dimension() as usize {
                return Err(TokenizerError::InvalidResponse(format!(
                    "embedding dimension mismatch: expected {}, got {}",
                    self.config.dimension(),
                    payload.embedding.len()
                )));
            }

            Ok(payload.into())
        };

        RetryIf::start(strategy, send, |err: &TokenizerError| err.is_retriable()).await
    }
}
