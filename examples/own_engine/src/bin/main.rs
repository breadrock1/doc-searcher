extern crate own_engine;

use doc_search::middlewares::logger::LoggerMiddlewareFactory;
use doc_search::services::init::*;
use doc_search::services::cacher::rediska::build_cacher_service;
use doc_search::services::elastic::service::*;
use doc_search::services::config::init_service_config;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let sv_params = init_service_config()?;

    let search_context = build_own_service(&sv_params)?;
    let redis_context = build_cacher_service(&sv_params)?;

    let service_port = sv_params.service_port();
    let service_addr = sv_params.service_address().as_str();
    let logger_host = sv_params.logger_service_host().to_owned();
    let cors_origin = sv_params.cors_origin().to_owned();

    HttpServer::new(move || {
        let searcher = search_context.clone();
        let clusters_cxt: Box<dyn ClustersService> = Box::new(searcher.clone());
        let folders_cxt: Box<dyn FoldersService> = Box::new(searcher.clone());
        let documents_cxt: Box<dyn DocumentsService> = Box::new(searcher.clone());
        let searcher_cxt: Box<dyn SearcherService> = Box::new(searcher);

        let redis = redis_context.clone();
        let redis_cxt = Box::new(redis);

        App::new()
            .app_data(web::Data::new(clusters_cxt))
            .app_data(web::Data::new(folders_cxt))
            .app_data(web::Data::new(documents_cxt))
            .app_data(web::Data::new(searcher_cxt))
            .app_data(web::Data::new(redis_cxt))
            .wrap(LoggerMiddlewareFactory::new(logger_host.as_str()))
            .wrap(Logger::default())
            .wrap(build_cors_config(cors_origin.as_str()))
            .service(build_hello_scope())
            .service(build_cluster_scope())
            .service(build_storage_scope())
            .service(build_search_scope())
            .service(build_pagination_scope())
            .service(build_watcher_scope())
    })
    .bind((service_addr, *service_port))?
    .workers(3)
    .run()
    .await?;

    Ok(())
}
