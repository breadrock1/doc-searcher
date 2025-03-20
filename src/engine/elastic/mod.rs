pub mod config;
mod from;
pub mod helper;
pub mod ops;
mod schema;
mod searcher;
mod storage;

use elasticsearch::auth::Credentials;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::response::Response;
use elasticsearch::http::transport::BuildError;
use elasticsearch::http::transport::SingleNodeConnectionPool;
use elasticsearch::http::transport::TransportBuilder;
use elasticsearch::http::{Method, Url};
use elasticsearch::{Elasticsearch, Error, SearchParts};
use getset::CopyGetters;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::engine::elastic::config::ElasticConfig;
use crate::engine::error::{SearcherError, SearcherResult};
use crate::errors::Successful;
use crate::ServiceConnect;

#[derive(Clone, CopyGetters)]
pub struct ElasticClient {
    client: Arc<RwLock<Elasticsearch>>,
}

#[async_trait::async_trait]
impl ServiceConnect for ElasticClient {
    type Config = ElasticConfig;
    type Error = BuildError;
    type Client = ElasticClient;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        let es_url = Url::parse(config.address()).unwrap();
        let conn_pool = SingleNodeConnectionPool::new(es_url);

        let es_user = config.username();
        let es_passwd = config.password();
        let creds = Credentials::Basic(es_user.into(), es_passwd.into());
        let validation = CertificateValidation::None;
        let transport = TransportBuilder::new(conn_pool)
            .auth(creds)
            .cert_validation(validation)
            .build()?;

        let elastic_core = Elasticsearch::new(transport);
        Ok(ElasticClient {
            client: Arc::new(RwLock::new(elastic_core)),
        })
    }
}

impl ElasticClient {
    pub async fn send_native_request(
        &self,
        method: Method,
        body: Option<&[u8]>,
        target_url: &str,
    ) -> Result<Response, Error> {
        let es_client = self.client.clone();
        let elastic = es_client.write().await;
        elastic
            .send(
                method,
                target_url,
                HeaderMap::new(),
                Option::<&Value>::None,
                body,
                None,
            )
            .await
    }

    pub async fn search_request(
        es: Arc<RwLock<Elasticsearch>>,
        query: &Value,
        scroll: Option<&str>,
        indexes: &[&str],
        result: (i64, i64),
    ) -> SearcherResult<Response> {
        let (size, offset) = result;
        let scroll = scroll.unwrap_or("1m");
        let elastic = es.read().await;
        let response = elastic
            .search(SearchParts::Index(indexes))
            .allow_no_indices(true)
            .pretty(true)
            .scroll(scroll)
            .from(offset)
            .size(size)
            .body(query)
            .send()
            .await?;

        let result = response
            .error_for_status_code()
            .map_err(SearcherError::from)?;
        Ok(result)
    }

    pub async fn search_knn_request(
        es: Arc<RwLock<Elasticsearch>>,
        query: &Value,
        scroll: Option<&str>,
        indexes: &[&str],
        size: i64,
    ) -> SearcherResult<Response> {
        let scroll = scroll.unwrap_or("1m");
        let elastic = es.read().await;
        let response = elastic
            .search(SearchParts::Index(indexes))
            .allow_no_indices(true)
            .pretty(true)
            .scroll(scroll)
            .size(size)
            .body(query)
            .send()
            .await?;

        let result = response
            .error_for_status_code()
            .map_err(SearcherError::from)?;
        Ok(result)
    }

    pub async fn extract_response_msg(response: Response) -> Result<Successful, Error> {
        let _ = response.error_for_status_code()?;
        Ok(Successful::new(200, "Done"))
    }
}
