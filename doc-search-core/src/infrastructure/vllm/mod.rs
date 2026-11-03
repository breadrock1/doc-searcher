#[cfg(test)]
mod tests;

mod config;
pub use config::VLLMConfig;

mod dto;
use dto::RerankedDocuments;
use dto::RerankerRequest;

mod error;
use error::map_http_error;
use error::map_openai_error;

use metrics::{counter, histogram};
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::embedding::{EmbeddingInput, EmbeddingOutput, EmbeddingParameters};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio_retry::RetryIf;
use tokio_retry::strategy::{ExponentialBackoff, jitter};
use tracing::instrument;

use crate::ServiceConnect;
use crate::application::services::embeddings::{ITokenizer, TokenizerError, TokenizerResult};
use crate::application::services::reranker::{IReranker, RerankerError, RerankerResult};
use crate::domain::searcher::models::FoundedDocument;
use crate::shared::kernel::Tokens;

const EXPONENTIAL_MIN_MILLS: u64 = 50;
const EXPONENTIAL_MAX_DELAY_SECS: u64 = 2;

#[derive(Clone)]
pub struct VLLMClient {
    config: VLLMConfig,
    client: Arc<Client>,
}

#[async_trait::async_trait]
impl ServiceConnect for VLLMClient {
    type Config = VLLMConfig;
    type Client = VLLMClient;
    type Error = reqwest::Error;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        let http_client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout()))
            .build()?;

        let mut client = Client::new(config.api_key().to_owned());
        client.set_base_url(config.base_url());
        client.http_client = http_client;

        tracing::debug!(base_url = %config.base_url(), "connected to vllm service");

        Ok(VLLMClient {
            config: config.clone(),
            client: Arc::new(client),
        })
    }
}

#[async_trait::async_trait]
impl ITokenizer for VLLMClient {
    #[instrument(level = "info", skip(self, chunk), fields(chunks_len = 1))]
    async fn compute(&self, chunk: String) -> TokenizerResult<Tokens> {
        let instant = Instant::now();

        let computed_result = self.compute_with_retries(vec![chunk]).await;
        let status = computed_result.is_ok().to_string();

        counter!("docsearch_embeddings_requests_total", "status" => status).increment(1);
        histogram!("docsearch_embeddings_request_duration_seconds")
            .record(instant.elapsed().as_secs_f64());

        let computed = computed_result?;
        computed
            .first()
            .cloned()
            .ok_or(TokenizerError::EmptyVectors)
    }

    #[instrument(level = "info", skip(self, chunks), fields(chunks_len = chunks.len()))]
    async fn compute_bulk(&self, chunks: Vec<String>) -> TokenizerResult<Vec<Tokens>> {
        let instant = Instant::now();

        let computed_result = self.compute_with_retries(chunks).await;
        let status = computed_result.is_ok().to_string();

        counter!("docsearch_embeddings_requests_total", "status" => status).increment(1);
        histogram!("docsearch_embeddings_request_duration_seconds")
            .record(instant.elapsed().as_secs_f64());

        computed_result
    }
}

#[async_trait::async_trait]
impl IReranker for VLLMClient {
    #[instrument(level = "info", skip(self, docs), fields(docs_len = docs.len()))]
    async fn rerank(
        &self,
        query: String,
        mut docs: Vec<FoundedDocument>,
    ) -> RerankerResult<Vec<FoundedDocument>> {
        if !self.config.enable() {
            return Ok(docs);
        }

        let instant = Instant::now();

        let chunks = docs
            .iter_mut()
            .map(|it| it.document.content.clone())
            .map(|it| it.unwrap_or_default())
            .collect();

        let result = self.rerank_with_retries(query, chunks).await;
        let status = result.is_ok().to_string();

        counter!("docsearch_rerank_requests_total", "status" => status).increment(1);
        histogram!("docsearch_rerank_request_duration_seconds")
            .record(instant.elapsed().as_secs_f64());

        let reranked = result?;
        reranked.results.into_iter().for_each(|it| {
            if let Some(doc) = docs.get_mut(it.index as usize) {
                doc.score = it.relevance_score
            }
        });

        docs.sort_by(|a, b| b.score.total_cmp(&a.score));

        Ok(docs)
    }
}

impl VLLMClient {
    async fn compute_with_retries(&self, chunks: Vec<String>) -> TokenizerResult<Vec<Tokens>> {
        let strategy = ExponentialBackoff::from_millis(EXPONENTIAL_MIN_MILLS)
            .max_delay(Duration::from_secs(EXPONENTIAL_MAX_DELAY_SECS))
            .map(jitter)
            .take(self.config.retries() as usize);

        let parameters = EmbeddingParameters {
            model: self.config.model().to_owned(),
            input: EmbeddingInput::StringArray(chunks.clone()),
            ..EmbeddingParameters::default()
        };

        let send = || async {
            let response = self
                .client
                .embeddings()
                .create(parameters.clone())
                .await
                .map_err(map_openai_error)?;

            let mut result_tokens = Vec::with_capacity(chunks.len());
            for item in response.data {
                let EmbeddingOutput::Float(vector) = item.embedding else {
                    tracing::error!("unsupported embedding format");
                    continue;
                };

                if vector.len() != self.config.dimension() as usize {
                    tracing::error!(
                        got = vector.len(),
                        expected = self.config.dimension(),
                        "unexpected dimension",
                    );
                    continue;
                }

                result_tokens.push(Tokens::from(vector));
            }

            Ok(result_tokens)
        };

        RetryIf::start(strategy, send, |err: &TokenizerError| err.is_retriable()).await
    }

    async fn rerank_with_retries(
        &self,
        query: String,
        chunks: Vec<String>,
    ) -> RerankerResult<RerankedDocuments> {
        let strategy = ExponentialBackoff::from_millis(EXPONENTIAL_MIN_MILLS)
            .max_delay(Duration::from_secs(EXPONENTIAL_MAX_DELAY_SECS))
            .map(jitter)
            .take(self.config.retries() as usize);

        let parameters = RerankerRequest {
            model: self.config.model().clone(),
            query,
            documents: chunks,
            top_n: self.config.top_n(),
            truncate_prompt_tokens: -1,
        };

        let target_url = format!("{}/rerank", self.config.base_url());
        let send = || async {
            let response = self
                .client
                .http_client
                .post(target_url.clone())
                .json(&parameters)
                .send()
                .await
                .map_err(map_http_error)?;

            let reranked = response
                .json::<RerankedDocuments>()
                .await
                .map_err(anyhow::Error::from)
                .map_err(RerankerError::InvalidResponse)?;

            Ok(reranked)
        };

        RetryIf::start(strategy, send, |err: &RerankerError| err.is_retriable()).await
    }
}
