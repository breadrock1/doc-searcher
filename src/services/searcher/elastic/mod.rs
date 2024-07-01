mod clusters;
pub mod context;
pub(crate) mod documents;
mod folders;
pub(crate) mod helper;
mod paginator;
mod searcher;

use crate::services::config::ServiceConfig;
use crate::services::searcher::elastic::context::ElasticContext;

use elasticsearch::auth::Credentials;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::transport::SingleNodeConnectionPool;
use elasticsearch::http::transport::{BuildError, TransportBuilder};
use elasticsearch::http::Url;
use elasticsearch::Elasticsearch;

pub type InitSearcherResult = Result<ElasticContext, BuildError>;

pub fn build_searcher_service(s_config: &ServiceConfig) -> InitSearcherResult {
    let es_url = Url::parse(s_config.get_elastic_host()).unwrap();
    let conn_pool = SingleNodeConnectionPool::new(es_url);

    let es_user = s_config.get_elastic_user();
    let es_passwd = s_config.get_elastic_passwd();
    let creds = Credentials::Basic(es_user.into(), es_passwd.into());
    let validation = CertificateValidation::None;
    let transport = TransportBuilder::new(conn_pool)
        .auth(creds)
        .cert_validation(validation)
        .build()?;

    let elastic_client = Elasticsearch::new(transport);
    Ok(ElasticContext::new(elastic_client, s_config))
}
