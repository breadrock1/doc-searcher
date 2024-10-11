use crate::Connectable;

use elasticsearch::auth::Credentials;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::transport::{BuildError, SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::http::Url;
use elasticsearch::Elasticsearch;
use getset::{CopyGetters, Getters};
use std::sync::Arc;
use serde_derive::Deserialize;
use tokio::sync::RwLock;

#[derive(Default, Clone, CopyGetters)]
pub struct ElasticClient {
    es_client: Arc<RwLock<Elasticsearch>>,
}

impl ElasticClient {
    pub fn es_client(&self) -> Arc<RwLock<Elasticsearch>> {
        self.es_client.clone()
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
