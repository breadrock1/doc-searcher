extern crate doc_search;

use doc_search::middlewares::logger::logger::LoggerMiddlewareFactory;
use doc_search::services::init::*;
use doc_search::services::{CacherClient, SearcherService};
use doc_search::services::own_engine::build_own_service;
use doc_search::swagger::{ApiDoc, OpenApi};
use doc_search::swagger::create_service;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let sv_params = init_service_parameters()?;

    let service_port = sv_params.service_port();
    let service_addr = sv_params.service_address();
    let logger_mw_addr = sv_params.logger_mw().to_owned();
    let cors_origin = sv_params.cors_origin().to_owned();
    let cacher_addr = sv_params.cacher_addr().to_owned();
    let cacher_expire = sv_params.cacher_expire();

    let search_context = build_own_service(&sv_params)
        .expect("Failed while initializing own search services");

    HttpServer::new(move || {
        let cxt = search_context.clone();
        let box_cxt: Box<dyn SearcherService> = Box::new(cxt);

        let cacher_cxt = CacherClient::new(cacher_addr.as_str(), cacher_expire);

        let openapi = ApiDoc::openapi();
        let cors = build_cors_config(cors_origin.as_str());

        App::new()
            .app_data(web::Data::new(box_cxt))
            .app_data(web::Data::new(cacher_cxt))
            .wrap(Logger::default())
            .wrap(LoggerMiddlewareFactory::new(logger_mw_addr.as_str()))
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
