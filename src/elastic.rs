use crate::errors::Successful;
use crate::searcher::errors::{SearcherError, SearcherResult};
use crate::Connectable;

use elasticsearch::auth::Credentials;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::response::Response;
use elasticsearch::http::transport::BuildError;
use elasticsearch::http::transport::SingleNodeConnectionPool;
use elasticsearch::http::transport::TransportBuilder;
use elasticsearch::http::{Method, Url};
use elasticsearch::{Elasticsearch, Error, SearchParts};
use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type EsCxt = Arc<RwLock<Elasticsearch>>;

#[derive(Default, Clone, CopyGetters)]
pub struct ElasticClient {
    es_client: Arc<RwLock<Elasticsearch>>,
}

#[derive(Clone, Deserialize, CopyGetters, Getters)]
#[getset(get = "pub")]
pub struct ElasticConfig {
    address: String,
    username: String,
    password: String,
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    enabled_tls: bool,
}

impl ElasticClient {
    pub fn es_client(&self) -> Arc<RwLock<Elasticsearch>> {
        self.es_client.clone()
    }

    pub async fn send_native_request(
        &self,
        method: Method,
        body: Option<&[u8]>,
        target_url: &str,
    ) -> Result<Response, Error> {
        let es_client = self.es_client();
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
        es: EsCxt,
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

        match response.error_for_status_code() {
            Ok(resp) => Ok(resp),
            Err(err) => {
                tracing::error!(err=?err, "failed response from elastic");
                Err(SearcherError::from(err))
            }
        }
    }

    pub async fn search_knn_request(
        es: EsCxt,
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

        match response.error_for_status_code() {
            Ok(resp) => Ok(resp),
            Err(err) => {
                tracing::error!(err=?err, "failed knn response from elastic");
                Err(SearcherError::from(err))
            }
        }
    }

    pub async fn extract_response_msg(response: Response) -> Result<Successful, Error> {
        let _ = response.error_for_status_code()?;
        Ok(Successful::new(200, "Done"))
    }
}

impl Connectable for ElasticClient {
    type Config = ElasticConfig;
    type Error = BuildError;
    type Service = ElasticClient;

    fn connect(config: &Self::Config) -> Result<Self::Service, Self::Error> {
        let http_protocol = match config.enabled_tls {
            true => "https://",
            false => "http://",
        };

        let es_address = format!("{http_protocol}{}", config.address());
        let es_url = Url::parse(&es_address).unwrap();
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
            es_client: Arc::new(RwLock::new(elastic_core)),
        })
    }
}
