extern crate doc_search;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use doc_search::cors::build_cors;
use doc_search::elastic::ElasticClient;
use doc_search::embeddings::native::EmbeddingsClient;
use doc_search::embeddings::EmbeddingsService;
use doc_search::logger::init_logger;
use doc_search::metrics::endpoints::build_scope as build_metrics_scope;
use doc_search::searcher::endpoints::build_scope as build_searcher_scope;
use doc_search::searcher::{PaginatorService, SearcherService};
use doc_search::storage::endpoints::build_scope as build_storage_scope;
use doc_search::storage::{DocumentService, FolderService};
use doc_search::{config, swagger, Connectable};

#[cfg(feature = "enable-cacher")]
use doc_search::cacher;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let s_config = config::ServiceConfig::new()?;

    let server_config = s_config.server();
    let cors_config = s_config.cors().clone();
    let logger_config = s_config.logger();
    init_logger(logger_config)?;

    let search_service = ElasticClient::connect(s_config.elastic())?;
    let embeddings_service = EmbeddingsClient::connect(s_config.embeddings())?;

    #[cfg(feature = "enable-cacher")]
    let cacher_service = cacher::redis::RedisClient::connect(s_config.cacher())?;

    HttpServer::new(move || {
        let cors = build_cors(&cors_config.clone());
        let logger = Logger::default();

        let documents_cxt: Box<dyn DocumentService> = Box::new(search_service.clone());
        let folders_cxt: Box<dyn FolderService> = Box::new(search_service.clone());
        let paginator_cxt: Box<dyn PaginatorService> = Box::new(search_service.clone());
        let searcher_cxt: Box<dyn SearcherService> = Box::new(search_service.clone());
        let embeddings_cxt: Box<dyn EmbeddingsService> = Box::new(embeddings_service.clone());

        let app = App::new()
            .app_data(web::Data::new(documents_cxt))
            .app_data(web::Data::new(folders_cxt))
            .app_data(web::Data::new(paginator_cxt))
            .app_data(web::Data::new(searcher_cxt))
            .app_data(web::Data::new(embeddings_cxt));

        #[cfg(feature = "enable-cacher")]
        let cacher_search_cxt: cacher::redis::SearchParamsCached = Box::new(cacher_service.clone());
        #[cfg(feature = "enable-cacher")]
        let cacher_paginate_cxt: cacher::redis::PaginatedCached = Box::new(cacher_service.clone());
        #[cfg(feature = "enable-cacher")]
        let app = app
            .app_data(web::Data::new(cacher_search_cxt))
            .app_data(web::Data::new(cacher_paginate_cxt));

        app.wrap(logger)
            .wrap(cors)
            .service(build_metrics_scope())
            .service(build_storage_scope())
            .service(build_searcher_scope())
            .service(swagger::build_swagger_service())
    })
    .bind((server_config.address().to_owned(), server_config.port()))?
    .workers(server_config.workers_num())
    .run()
    .await?;

    Ok(())
}
