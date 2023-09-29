mod context;
mod endpoints;
mod errors;
mod wrappers;

use crate::context::SearchContext;
use crate::endpoints::elastic::{create_index, find_index};

use actix_web::{web, App, HttpServer, Scope};
use elasticsearch::{Elasticsearch, SearchParts};
use elasticsearch::auth::Credentials;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::transport::{SingleNodeConnectionPool, Transport, TransportBuilder};
use elasticsearch::http::Url;

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
    let creds = Credentials::Basic("elastic".into(), "MomQSpuUJk0ANHBTjSKM".into());
    let validation = CertificateValidation::None;
    let transport = TransportBuilder::new(conn_pool)
        .auth(creds)
        .cert_validation(validation)
        .build()
        .unwrap();

    Some(Elasticsearch::new(transport))
}

fn build_service() -> Scope {
    web::scope("/home")
        .service(create_index)
        .service(find_index)
}
