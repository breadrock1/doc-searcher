#[cfg(test)]
mod tests;

mod config;
pub use config::EmbeddingsConfig;

mod error;
use error::map_openai_error;

use metrics::{counter, histogram};
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::embedding::{EmbeddingInput, EmbeddingOutput, EmbeddingParameters};
use std::sync::Arc;
use std::time::{Duration, Instant};
use anyhow::anyhow;
use tokio_retry::RetryIf;
use tokio_retry::strategy::{ExponentialBackoff, jitter};
use tracing::instrument;

use crate::ServiceConnect;
use crate::application::services::embeddings::{ITokenizer, TokenizerError, TokenizerResult};
use crate::shared::kernel::Tokens;

const EXPONENTIAL_MIN_MILLS: u64 = 50;
const EXPONENTIAL_MAX_DELAY_SECS: u64 = 2;

#[derive(Clone)]
pub struct EmbeddingsClient {
    config: EmbeddingsConfig,
    client: Arc<Client>,
}

#[async_trait::async_trait]
impl ServiceConnect for EmbeddingsClient {
    type Config = EmbeddingsConfig;
    type Client = EmbeddingsClient;
    type Error = reqwest::Error;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        let http_client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout()))
            .build()?;

        let mut client = Client::new(config.api_key().to_owned());
        client.set_base_url(config.base_url());
        client.http_client = http_client;

        tracing::debug!(base_url = %config.base_url(), "connected to embeddings service");

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

        let computed_result = self.compute_with_retries(text).await;
        let status = computed_result.is_ok().to_string();

        counter!("docsearch_embeddings_requests_total", "status" => status).increment(1);
        histogram!("docsearch_embeddings_request_duration_seconds")
            .record(instant.elapsed().as_secs_f64());

        computed_result
    }
}

impl EmbeddingsClient {
    async fn compute_with_retries(&self, text: &str) -> TokenizerResult<Tokens> {
        let strategy = ExponentialBackoff::from_millis(EXPONENTIAL_MIN_MILLS)
            .max_delay(Duration::from_secs(EXPONENTIAL_MAX_DELAY_SECS))
            .map(jitter)
            .take(self.config.retries() as usize);

        let send = || async {
            let parameters = EmbeddingParameters {
                input: EmbeddingInput::String(text.to_owned()),
                model: self.config.model().to_owned(),
                dimensions: Some(self.config.dimension()),
                ..EmbeddingParameters::default()
            };

            let response = self
                .client
                .embeddings()
                .create(parameters)
                .await
                .map_err(map_openai_error)?;

            let embedding = response
                .data
                .into_iter()
                .next()
                .map(|item| item.embedding)
                .ok_or(anyhow!("empty embeddings"))
                .map_err(TokenizerError::InvalidResponse)?;

            let EmbeddingOutput::Float(vector) = embedding else {
                let err = anyhow!("unsupported embeddings format");
                return Err(TokenizerError::InvalidResponse(err));
            };

            if vector.len() != self.config.dimension() as usize {
                let err = anyhow!(
                    "expected dimension: {}, got {}",
                    self.config.dimension(),
                    vector.len(),
                );

                return Err(TokenizerError::InvalidResponse(err));
            }

            Ok(Tokens::from(vector))
        };

        RetryIf::start(strategy, send, |err: &TokenizerError| err.is_retriable()).await
    }
}
