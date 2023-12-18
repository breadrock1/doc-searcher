pub mod client;
pub mod context;
pub mod helper;
mod send_status;
mod query_builder;

use elasticsearch::auth::Credentials;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::transport::{BuildError, SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::http::Url;
use elasticsearch::Elasticsearch;

pub type ElasticBuildResult = Result<Elasticsearch, BuildError>;

pub fn build_elastic_client(es_host: &str, es_user: &str, es_passwd: &str) -> ElasticBuildResult {
    let es_url = Url::parse(es_host).unwrap();
    let conn_pool = SingleNodeConnectionPool::new(es_url);
    let creds = Credentials::Basic(es_user.into(), es_passwd.into());
    let validation = CertificateValidation::None;
    let transport = TransportBuilder::new(conn_pool)
        .auth(creds)
        .cert_validation(validation)
        .build()?;

    Ok(Elasticsearch::new(transport))
}
