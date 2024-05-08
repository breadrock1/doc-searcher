pub mod client;
pub mod context;
pub mod helper;
mod send_status;
pub mod watcher;

use crate::services::elastic::context::ElasticContext;
use crate::services::init::ServiceParameters;

use elasticsearch::auth::Credentials;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::transport::SingleNodeConnectionPool;
use elasticsearch::http::transport::{BuildError, TransportBuilder};
use elasticsearch::http::Url;
use elasticsearch::Elasticsearch;

pub type ElasticBuildResult = Result<Elasticsearch, BuildError>;

pub fn build_elastic_service(
    sv_params: &ServiceParameters,
) -> Result<ElasticContext, anyhow::Error> {
    let es_url = Url::parse(sv_params.es_host()).unwrap();
    let conn_pool = SingleNodeConnectionPool::new(es_url);
    let creds = Credentials::Basic(sv_params.es_user().into(), sv_params.es_passwd().into());
    let validation = CertificateValidation::None;
    let transport = TransportBuilder::new(conn_pool)
        .auth(creds)
        .cert_validation(validation)
        .build()?;

    Ok(ElasticContext::new(
        Elasticsearch::new(transport),
        sv_params,
    ))
}
