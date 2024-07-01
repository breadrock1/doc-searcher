use crate::endpoints::*;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, Scope};

pub fn build_cors_config(_origin: &str) -> Cors {
    let available_methods = vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"];
    let available_headers = vec![header::AUTHORIZATION, header::ACCEPT];

    Cors::default()
        .allowed_header(header::CONTENT_TYPE)
        .allowed_methods(available_methods)
        .allowed_headers(available_headers)
        .allow_any_origin()
        .max_age(3600)
}

pub fn build_env_logger() {
    let env_log = env_logger::Env::new();
    let env_log = env_log.default_filter_or("info");
    std::env::set_var("RUST_LOG", "info");
    env_logger::init_from_env(env_log);
}

pub fn build_hello_scope() -> Scope {
    web::scope("/hello").service(hello::hello)
}

pub fn build_cluster_scope() -> Scope {
    web::scope("/orchestra")
        .service(clusters::create_cluster)
        .service(clusters::delete_cluster)
        .service(clusters::get_clusters)
        .service(clusters::get_cluster)
}

pub fn build_storage_scope() -> Scope {
    web::scope("/storage")
        .service(folders::get_folders)
        .service(folders::get_folder)
        .service(folders::create_folder)
        .service(folders::delete_folder)
        .service(documents::get_document)
        .service(documents::create_document)
        .service(documents::delete_document)
        .service(documents::update_document)
        .service(searcher::get_index_records)
}

pub fn build_search_scope() -> Scope {
    web::scope("/search")
        .service(searcher::search_fulltext)
        .service(searcher::search_semantic)
}

pub fn build_pagination_scope() -> Scope {
    web::scope("/search")
        .service(paginator::delete_paginate_sessions)
        .service(paginator::paginate_next)
}
