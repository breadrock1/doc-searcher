mod clusters;
pub mod context;
mod documents;
mod folders;
mod helper;
mod paginator;
mod searcher;
mod watcher;

use crate::services::elastic::context::ElasticContext;
use crate::services::init::ServiceParameters;

use elasticsearch::auth::Credentials;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::transport::SingleNodeConnectionPool;
use elasticsearch::http::transport::{BuildError, TransportBuilder};
use elasticsearch::http::Url;
use elasticsearch::Elasticsearch;

pub type InitElasticResult = Result<ElasticContext, BuildError>;

pub fn build_elastic_service(sv_params: &ServiceParameters) -> InitElasticResult {
    let es_url = Url::parse(sv_params.es_host()).unwrap();
    let conn_pool = SingleNodeConnectionPool::new(es_url);
    let creds = Credentials::Basic(sv_params.es_user().into(), sv_params.es_passwd().into());
    let validation = CertificateValidation::None;
    let transport = TransportBuilder::new(conn_pool)
        .auth(creds)
        .cert_validation(validation)
        .build()?;

    let elastic_client = Elasticsearch::new(transport);
    Ok(ElasticContext::new(elastic_client, sv_params))
}
