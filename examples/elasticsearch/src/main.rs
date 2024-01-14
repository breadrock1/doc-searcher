extern crate docsearcher;
use docsearcher::service::*;
use docsearcher::searcher::service_client::ServiceClient;
use docsearcher::searcher::elastic::context::ElasticContext;

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
    let search_context = build_elastic_service(es_host, es_user, es_passwd);

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

fn build_elastic_service(es_host: &str, es_user: &str, es_passwd: &str) -> ElasticContext {
    use docsearcher::searcher::elastic::build_elastic_client;
    let client = build_elastic_client(es_host, es_user, es_passwd);
    ElasticContext::_new(client.unwrap())
}
