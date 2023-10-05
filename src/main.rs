mod context;
mod endpoints;
mod errors;
mod wrappers;

use crate::context::SearchContext;
use crate::endpoints::buckets::{all_buckets, delete_bucket, get_bucket, new_bucket};
use crate::endpoints::clusters::{all_clusters, delete_cluster, get_cluster, new_cluster};
use crate::endpoints::documents::{delete_document, get_document, new_document, update_document};
use crate::endpoints::searcher::{search_all, search_target};

use actix_web::{web, App, HttpServer, Scope};
use elasticsearch::auth::Credentials;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::http::Url;
use elasticsearch::Elasticsearch;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let elastic = build_elastic().unwrap();
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

fn build_elastic() -> Option<Elasticsearch> {
    let es_url = Url::parse("https://localhost:9200").unwrap();
    let conn_pool = SingleNodeConnectionPool::new(es_url);
    let creds = Credentials::Basic("elastic".into(), "s4Tvs7hAtki_ME_fNUuo".into());
    let validation = CertificateValidation::None;
    let transport = TransportBuilder::new(conn_pool)
        .auth(creds)
        .cert_validation(validation)
        .build()
        .unwrap();

    Some(Elasticsearch::new(transport))
}

fn build_service() -> Scope {
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
