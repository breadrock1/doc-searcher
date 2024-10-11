use crate::embeddings::config::EmbeddingsConfig;
use crate::embeddings::EmbeddingsLoader;
use crate::Connectable;

use getset::{CopyGetters, Getters};
use serde_json::json;
use std::sync::Arc;

const NATIVE_SERVICE_URL: &str = "/embed";

#[derive(CopyGetters, Getters)]
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

impl EmbeddingsLoader for EmbeddingsClient {
    type Error = reqwest::Error;

    async fn load_from_text(&self, text: &str) -> Result<Vec<f64>, Self::Error> {
        let client_addr = self.address();
        let target_url = format!("{client_addr}{NATIVE_SERVICE_URL}");
        let response = self.client()
            .clone()
            .post(target_url)
            .json(&json!({
                "inputs": text,
                "truncate": self.is_truncate(),
                "normalize": self.is_normalize(),
            }))
            .send()
            .await?;

        let embed_data = response.json::<Vec<Vec<f64>>>().await?;
        let slice = embed_data.first().unwrap().to_owned();
        Ok(slice)
    }
}
