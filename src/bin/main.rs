extern crate doc_search;

use doc_search::services::cacher::rediska;
use doc_search::services::config;
use doc_search::services::init;
use doc_search::services::searcher::elastic;
use doc_search::services::searcher::service::*;
use doc_search::services::swagger;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let s_config = config::init_service_config()?;

    let search_service = elastic::build_searcher_service(&s_config)?;
    let cacher_service = rediska::build_cacher_service(&s_config)?;

    let service_port = *s_config.service_port();
    let service_addr = s_config.service_host().as_str();
    let cors_origin = s_config.cors_origin().to_owned();
    let workers_num = *s_config.workers_num();

    HttpServer::new(move || {
        let cacher_cxt = Box::new(cacher_service.clone());

        let searcher = search_service.clone();
        let clusters_cxt: Box<dyn ClusterService> = Box::new(searcher.clone());
        let documents_cxt: Box<dyn DocumentService> = Box::new(searcher.clone());
        let folders_cxt: Box<dyn FolderService> = Box::new(searcher.clone());
        let paginator_cxt: Box<dyn PaginatorService> = Box::new(searcher.clone());
        let searcher_cxt: Box<dyn SearcherService> = Box::new(searcher.clone());
        let watcher_cxt: Box<dyn WatcherService> = Box::new(searcher);

        App::new()
            .app_data(web::Data::new(clusters_cxt))
            .app_data(web::Data::new(documents_cxt))
            .app_data(web::Data::new(folders_cxt))
            .app_data(web::Data::new(paginator_cxt))
            .app_data(web::Data::new(cacher_cxt))
            .app_data(web::Data::new(searcher_cxt))
            .app_data(web::Data::new(watcher_cxt))
            .wrap(Logger::default())
            .wrap(init::build_cors_config(cors_origin.as_str()))
            .service(init::build_hello_scope())
            .service(init::build_cluster_scope())
            .service(init::build_storage_scope())
            .service(init::build_search_scope())
            .service(init::build_pagination_scope())
            .service(init::build_watcher_scope())
            .service(swagger::build_swagger_service())
    })
    .bind((service_addr, service_port))?
    .workers(workers_num)
    .run()
    .await?;

    Ok(())
}
