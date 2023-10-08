use crate::endpoints::buckets::{all_buckets, delete_bucket, get_bucket, new_bucket};
use crate::endpoints::clusters::{all_clusters, delete_cluster, get_cluster, new_cluster};
use crate::endpoints::documents::{delete_document, get_document, new_document, update_document};
use crate::endpoints::searcher::{search_all, search_target};

use actix_web::{web, Scope};
use dotenv::dotenv;
use elasticsearch::auth::Credentials;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::transport::{BuildError, SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::http::Url;
use elasticsearch::Elasticsearch;
use std::env::var;
use std::str::FromStr;

pub fn build_elastic(
    es_host: &str,
    es_user: &str,
    es_passwd: &str,
) -> Result<Elasticsearch, BuildError> {
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

pub fn build_service() -> Scope {
    web::scope("/searcher")
        .service(new_cluster)
        .service(delete_cluster)
        .service(all_clusters)
        .service(get_cluster)
        .service(new_bucket)
        .service(delete_bucket)
        .service(all_buckets)
        .service(get_bucket)
        .service(new_document)
        .service(delete_document)
        .service(update_document)
        .service(get_document)
        .service(search_target)
        .service(search_all)
}

pub struct ServiceParameters {
    es_host: String,
    es_user: String,
    es_passwd: String,
    service_addr: String,
    service_port: u16,
}

impl ServiceParameters {
    pub fn new(
        es_host: String,
        es_user: String,
        es_passwd: String,
        service_addr: String,
        service_port: u16,
    ) -> Self {
        ServiceParameters {
            es_host,
            es_user,
            es_passwd,
            service_addr,
            service_port,
        }
    }

    pub fn es_host(&self) -> &str {
        self.es_host.as_str()
    }

    pub fn es_user(&self) -> &str {
        self.es_user.as_str()
    }

    pub fn es_passwd(&self) -> &str {
        self.es_passwd.as_str()
    }

    pub fn service_address(&self) -> &str {
        self.service_addr.as_str()
    }

    pub fn service_port(&self) -> u16 {
        self.service_port
    }
}

pub fn init_service_parameters() -> Result<ServiceParameters, BuildError> {
    dotenv().ok();

    let es_host = var("ELASTIC_HOST").expect("There is not ELASTIC_HOST env variable!");
    let es_user = var("ELASTIC_USER").expect("There is no ELASTIC_USER env variable!");
    let es_passwd = var("ELASTIC_PASSWORD").expect("There is not ELASTIC_PASSWORD env variable!");
    let client_addr = var("SEARCHER_ADDRESS").expect("There is not SEARCHER_ADDRESS env variable!");
    let client_port = var("SEARCHER_PORT").expect("There is not SEARCHER_PORT env variable!");
    let client_port =
        u16::from_str(client_port.as_str()).expect("Failed while parsing port number.");

    let service = ServiceParameters::new(es_host, es_user, es_passwd, client_addr, client_port);
    Ok(service)
}
