extern crate doc_search;

use doc_search::middlewares::*;
use doc_search::service::init::*;
use doc_search::swagger::ApiDoc;
use doc_search::swagger::OpenApi;
use doc_search::service::ServiceClient;
use doc_search::swagger::create_service;
use doc_search::service::elastic::context::ElasticContext;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let sv_params = init_service_parameters()?;

    let service_port = sv_params.service_port();
    let service_addr = sv_params.service_address();
    let logger_mw_addr = sv_params.logger_mw().to_owned();
    let cors_origin = sv_params.cors_origin().to_owned();

    let search_context = build_elastic_service(&sv_params);

    HttpServer::new(move || {
        let cxt = search_context.clone();
        let box_cxt: Box<dyn ServiceClient> = Box::new(cxt);

        let openapi = ApiDoc::openapi();
        let cors = build_cors_config(cors_origin.as_str());

        App::new()
            .app_data(web::Data::new(box_cxt))
            .wrap(Logger::default())
            .wrap(logger::LoggerMiddlewareFactory::new(logger_mw_addr.as_str()))
            .wrap(cors)
            .service(create_service(&openapi))
            .service(build_hello_scope())
            .service(build_cluster_scope())
            .service(build_bucket_scope())
            .service(build_document_scope())
            .service(build_search_scope())
            .service(build_similar_scope())
            .service(build_file_scope())
    })
        .bind((service_addr, service_port))?
        .run()
        .await?;

    Ok(())
}

fn build_elastic_service(service_params: &ServiceParameters) -> ElasticContext {
    use doc_search::service::elastic::build_elastic_client;
    let client = build_elastic_client(
        service_params.es_host(),
        service_params.es_user(),
        service_params.es_passwd(),
    );

    ElasticContext::_new(client.unwrap())
}
