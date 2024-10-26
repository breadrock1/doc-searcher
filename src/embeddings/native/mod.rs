use crate::embeddings::config::EmbeddingsConfig;
use crate::embeddings::errors::{EmbeddingsError, EmbeddingsResult};
use crate::embeddings::EmbeddingsService;
use crate::Connectable;

use getset::{CopyGetters, Getters};
use serde_json::json;
use std::sync::Arc;

const NATIVE_SERVICE_URL: &str = "/embed";

#[derive(Clone, CopyGetters, Getters)]
pub struct EmbeddingsClient {
    #[getset(get = "pub")]
    address: String,
    #[getset(get_copy = "pub")]
    is_normalize: bool,
    #[getset(get_copy = "pub")]
    is_truncate: bool,
    client: Arc<reqwest::Client>,
}

impl EmbeddingsClient {
    pub fn client(&self) -> Arc<reqwest::Client> {
        self.client.clone()
    }
}

impl Connectable for EmbeddingsClient {
    type Config = EmbeddingsConfig;
    type Error = reqwest::Error;
    type Service = EmbeddingsClient;

    fn connect(config: &Self::Config) -> Result<Self::Service, Self::Error> {
        let http_protocol = match config.enabled_tls() {
            true => "https://",
            false => "http://",
        };

        let service_url = format!("{http_protocol}{}", config.address());

        Ok(EmbeddingsClient {
            address: service_url,
            is_normalize: config.is_normalize(),
            is_truncate: config.is_truncate(),
            client: Arc::new(reqwest::Client::new()),
        })
    }
}

#[async_trait::async_trait]
impl EmbeddingsService for EmbeddingsClient {
    async fn load_from_text(&self, text: &str) -> EmbeddingsResult<Vec<f64>> {
        let client_addr = self.address();
        let target_url = format!("{client_addr}{NATIVE_SERVICE_URL}");
        let response = self
            .client()
            .clone()
            .post(target_url)
            .json(&json!({
                "inputs": text,
                "truncate": self.is_truncate(),
                "normalize": self.is_normalize(),
            }))
            .send()
            .await
            .map_err(EmbeddingsError::from)?;

        let embed_data = response
            .json::<Vec<Vec<f64>>>()
            .await
            .map_err(EmbeddingsError::from)?;

        let Some(tokens) = embed_data.first() else {
            let msg = "loaded empty tokens array";
            return Err(EmbeddingsError::ServiceError(msg.to_string()));
        };

        Ok(tokens.to_owned())
    }
}
