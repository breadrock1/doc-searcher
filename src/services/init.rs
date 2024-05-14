use crate::endpoints::*;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, Scope};
use derive_builder::Builder;
use derive_getters::Getters;

#[derive(Builder, Getters)]
pub struct ServiceParameters {
    service_address: String,
    service_port: u16,
    es_host: String,
    es_user: String,
    es_passwd: String,
    cacher_service_host: String,
    cacher_expire: u64,
    llm_service_host: String,
    watcher_service_host: String,
    logger_service_host: String,
    ocr_service_host: String,
    cors_origin: String,
    global_folders: String,
}

pub fn init_service_parameters() -> Result<ServiceParameters, anyhow::Error> {
    #[cfg(feature = "enable-dotenv")]
    {
        use dotenv::dotenv;
        dotenv().ok();
    }

    build_env_logger();

    let service = ServiceParametersBuilder::default()
        .service_address(extract_env_value("SEARCHER_HOST"))
        .service_port(extract_int_env_value::<u16>("SEARCHER_PORT"))
        .es_host(extract_env_value("ELASTIC_SERVICE_HOST"))
        .es_user(extract_env_value("ELASTIC_SERVICE_USERNAME"))
        .es_passwd(extract_env_value("ELASTIC_SERVICE_PASSWORD"))
        .cacher_service_host(extract_env_value("CACHER_SERVICE_HOST"))
        .cacher_expire(extract_int_env_value::<u64>("CACHER_EXPIRE"))
        .llm_service_host(extract_env_value("LLM_SERVICE_HOST"))
        .watcher_service_host(extract_env_value("WATCHER_SERVICE_HOST"))
        .logger_service_host(extract_env_value("LOGGER_SERVICE_HOST"))
        .ocr_service_host(extract_env_value("OCR_SERVICE_HOST"))
        .cors_origin(extract_env_value("CORS_ORIGIN"))
        .global_folders(extract_env_value("GLOBAL_FOLDERS"))
        .build();

    Ok(service.unwrap())
}

fn extract_env_value(env_var: &str) -> String {
    let env_var_res = std::env::var(env_var);
    if env_var_res.is_err() {
        panic!("Env variable {} hasn't been founded!", env_var)
    }
    env_var_res.unwrap()
}

fn extract_int_env_value<T>(env_var: &str) -> T
where
    T: std::str::FromStr + std::fmt::Debug,
    T::Err: std::fmt::Debug,
{
    let env_var_val = extract_env_value(env_var);
    let env_var_res = T::from_str(env_var_val.as_str());
    if env_var_res.is_err() {
        let err = env_var_res.err().unwrap();
        panic!("Failed while parsing {} from env var: {:?}", env_var, err);
    }
    env_var_res.ok().unwrap()
}

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
    env_logger::init_from_env(env_log);
}

pub fn build_hello_scope() -> Scope {
    web::scope("/hello").service(hello::hello)
}

pub fn build_cluster_scope() -> Scope {
    web::scope("/clusters")
        .service(clusters::create_cluster)
        .service(clusters::delete_cluster)
        .service(clusters::all_clusters)
        .service(clusters::get_cluster)
}

pub fn build_folder_scope() -> Scope {
    web::scope("/folders")
        .service(folders::all_folders)
        .service(folders::create_folder)
        .service(folders::delete_folder)
        .service(folders::get_folder)
        .service(folders::get_folder_documents)
}

pub fn build_document_scope() -> Scope {
    web::scope("/documents")
        .service(documents::create_document)
        .service(documents::delete_documents)
        .service(documents::update_document)
        .service(documents::get_document)
}

pub fn build_search_scope() -> Scope {
    web::scope("/search")
        .service(searcher::search_all)
        .service(searcher::search_tokens)
        .service(searcher::search_similar_docs)
}

pub fn build_pagination_scope() -> Scope {
    web::scope("/pagination")
        .service(paginator::get_pagination_ids)
        .service(paginator::delete_expired_ids)
        .service(paginator::next_pagination_result)
}

pub fn build_watcher_scope() -> Scope {
    web::scope("/watcher")
        .service(watcher::analyse_documents)
        .service(watcher::upload_files)
}
