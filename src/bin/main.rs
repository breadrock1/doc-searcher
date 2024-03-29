extern crate doc_search;

use doc_search::middlewares::logger::LoggerMiddlewareFactory;
use doc_search::services::cacher::build_redis_service;
use doc_search::services::elastic::build_elastic_service;
use doc_search::services::init::*;
use doc_search::services::SearcherService;
use doc_search::swagger::create_service;
use doc_search::swagger::{ApiDoc, OpenApi};

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let sv_params = init_service_parameters()?;

    let service_port = sv_params.service_port();
    let service_addr = sv_params.service_address();
    let logger_mw_addr = sv_params.logger_mw().to_owned();
    let cors_origin = sv_params.cors_origin().to_owned();

    let search_context =
        build_elastic_service(&sv_params).expect("Failed while initializing elasticsearch client!");

    let redis_context =
        build_redis_service(&sv_params).expect("Failed while initializing redis client");

    HttpServer::new(move || {
        let searcher = search_context.clone();
        let searcher_cxt: Box<dyn SearcherService> = Box::new(searcher);

        let redis = redis_context.clone();
        let redis_cxt = Box::new(redis);

        let openapi = ApiDoc::openapi();
        let cors = build_cors_config(cors_origin.as_str());

        App::new()
            .app_data(web::Data::new(searcher_cxt))
            .app_data(web::Data::new(redis_cxt))
            .wrap(LoggerMiddlewareFactory::new(logger_mw_addr.as_str()))
            .wrap(Logger::default())
            .wrap(cors)
            .service(create_service(&openapi))
            .service(build_hello_scope())
            .service(build_cluster_scope())
            .service(build_bucket_scope())
            .service(build_document_scope())
            .service(build_search_scope())
            .service(build_similar_scope())
            .service(build_pagination_scope())
            .service(build_file_scope())
    })
    .bind((service_addr, service_port))?
    .run()
    .await?;

    Ok(())
}
