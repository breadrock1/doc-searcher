pub mod context;
pub mod endpoints;
pub mod errors;
pub mod wrappers;

use crate::context::SearchContext;
use crate::endpoints::buckets::{all_buckets, delete_bucket, get_bucket, new_bucket};
use crate::endpoints::clusters::{all_clusters, delete_cluster, get_cluster, new_cluster};
use crate::endpoints::documents::{delete_document, get_document, new_document, update_document};
use crate::endpoints::searcher::{search_all, search_target};

use std::env::var;

use actix_web::{web, App, HttpServer, Scope};
use elasticsearch::auth::Credentials;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::http::Url;
use elasticsearch::Elasticsearch;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let es_host = var("ELASTIC_HOST")
        .expect("There is not ELASTIC_HOST env variable!");
    let es_user = var("ELASTIC_USER")
        .expect("There is no ELASTIC_USER env variable!");
    let es_passwd = var("ELASTIC_PASSWORD")
        .expect("There is not ELASTIC_PASSWORD env variable!");

    let elastic = build_elastic(es_host.as_str(), es_user.as_str(), es_passwd.as_str())
        .ok_or(std::io::ErrorKind::ConnectionReset)?;

    let cxt = SearchContext::_new(elastic).unwrap();
    HttpServer::new(move || {
        let cxt = cxt.clone();
        App::new()
            .app_data(web::Data::new(cxt))
            .service(build_service())
    })
    .bind(("127.0.0.1", 45678))?
    .run()
    .await
}

pub fn build_elastic(es_host: &str, es_user: &str, es_passwd: &str) -> Option<Elasticsearch> {
    let es_url = Url::parse(es_host).unwrap();
    let conn_pool = SingleNodeConnectionPool::new(es_url);
    let creds = Credentials::Basic(es_user.into(), es_passwd.into());
    let validation = CertificateValidation::None;
    let transport_result = TransportBuilder::new(conn_pool)
        .auth(creds)
        .cert_validation(validation)
        .build();

    match transport_result {
        Ok(transport) => Some(Elasticsearch::new(transport)),
        Err(err) => {
            println!("{:?}", err);
            None
        }
    }
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
