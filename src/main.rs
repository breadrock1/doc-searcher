mod endpoints;
mod errors;
mod searcher;
mod service;
mod wrappers;

use crate::searcher::elastic::context::ElasticContext;
use crate::searcher::own_engine::context::OtherContext;
use crate::searcher::service_client::ServiceClient;
use crate::service::{build_cors_config, build_service, init_service_parameters};

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let service_parameters = init_service_parameters()?;
    let es_host = service_parameters.es_host();
    let es_user = service_parameters.es_user();
    let es_passwd = service_parameters.es_passwd();
    let service_port = service_parameters.service_port();
    let service_addr = service_parameters.service_address();
    let cors_origin = service_parameters.cors_origin();

    #[cfg(feature = "elastic-search")]
    let search_context = build_elastic_service(es_host, es_user, es_passwd);

    #[cfg(feature = "default-search")]
    let search_context = build_client_service(es_host, es_user, es_passwd);

    HttpServer::new(move || {
        let cxt = search_context.clone();
        let box_cxt: Box<dyn ServiceClient> = Box::new(cxt);
        let cors_cln = cors_origin.clone();
        let cors = build_cors_config(cors_cln.as_str());
        App::new()
            .app_data(web::Data::new(box_cxt))
            .service(build_service())
            .wrap(Logger::default())
            .wrap(cors)
    })
    .bind((service_addr, service_port))?
    .run()
    .await?;

    Ok(())
}

#[cfg(feature = "elastic-search")]
fn build_elastic_service(es_host: &str, es_user: &str, es_passwd: &str) -> ElasticContext {
    use crate::searcher::elastic::build_elastic_client;
    let client = build_elastic_client(es_host, es_user, es_passwd);
    ElasticContext::_new(client.unwrap())
}

#[cfg(feature = "default-search")]
fn build_client_service(es_host: &str, es_user: &str, es_passwd: &str) -> OtherContext {
    use crate::searcher::own_engine::build_own_client;
    build_own_client(es_host, es_user, es_passwd)?
}
