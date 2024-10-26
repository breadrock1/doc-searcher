use crate::errors::Successful;
use crate::Connectable;

use crate::searcher::errors::SearcherResult;
use elasticsearch::auth::Credentials;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::response::Response;
use elasticsearch::http::transport::{BuildError, SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::http::{Method, Url};
use elasticsearch::{Elasticsearch, SearchParts};
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
pub struct ElasticConfig {
    #[getset(get = "pub")]
    address: String,
    #[getset(get_copy = "pub")]
    enabled_tls: bool,
    #[getset(get = "pub")]
    username: String,
    #[getset(get = "pub")]
    password: String,
}

impl ElasticClient {
    pub fn es_client(&self) -> Arc<RwLock<Elasticsearch>> {
        self.es_client.clone()
    }

    pub async fn send_request(
        &self,
        method: Method,
        body: Option<&[u8]>,
        target_url: &str,
    ) -> Result<Response, elasticsearch::Error> {
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
        indexes: &[&str],
        result: (i64, i64),
    ) -> SearcherResult<Response> {
        let (size, offset) = result;
        let elastic = es.read().await;
        let response = elastic
            .search(SearchParts::Index(indexes))
            .from(offset)
            .size(size)
            .body(query)
            .pretty(true)
            .allow_no_indices(true)
            .send()
            .await?;

        let response = response.error_for_status_code()?;
        Ok(response)
    }

    pub async fn extract_response_msg(
        response: Response,
    ) -> Result<Successful, elasticsearch::Error> {
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
