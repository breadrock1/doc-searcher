use crate::endpoints::buckets;
use crate::endpoints::clusters;
use crate::endpoints::documents;
use crate::endpoints::hello::hello;
use crate::endpoints::loader;
use crate::endpoints::paginator;
use crate::endpoints::searcher;
use crate::endpoints::similarities::search_similar_docs;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, Scope};
use derive_builder::Builder;

use std::env::var;
use std::str::FromStr;

#[derive(Builder)]
pub struct ServiceParameters {
    es_host: String,
    es_user: String,
    es_passwd: String,
    service_addr: String,
    service_port: u16,
    cors_origin: String,
    logger_mw_addr: String,
    cacher_addr: String,
    cacher_expire: u64,
    embeddings_url: String,
}

impl ServiceParameters {
    pub fn es_host(&self) -> &str {
        self.es_host.as_str()
    }

    pub fn es_user(&self) -> &str {
        self.es_user.as_str()
    }

    pub fn es_passwd(&self) -> &str {
        self.es_passwd.as_str()
    }

    pub fn service_address(&self) -> &str {
        self.service_addr.as_str()
    }

    pub fn cors_origin(&self) -> &str {
        self.cors_origin.as_str()
    }

    pub fn logger_mw(&self) -> &str {
        self.logger_mw_addr.as_str()
    }

    pub fn cacher_addr(&self) -> &str {
        self.cacher_addr.as_str()
    }

    pub fn service_port(&self) -> u16 {
        self.service_port
    }

    pub fn cacher_expire(&self) -> u64 {
        self.cacher_expire
    }

    pub fn embeddings_url(&self) -> &str {
        self.embeddings_url.as_str()
    }
}

pub fn init_service_parameters() -> Result<ServiceParameters, anyhow::Error> {
    #[cfg(feature = "enable-dotenv")]
    {
        use dotenv::dotenv;
        dotenv().ok();
    }

    build_env_logger();

    let es_host = var("ELASTIC_HOST").expect("There is not ELASTIC_HOST env variable!");
    let es_user = var("ELASTIC_USER").expect("There is no ELASTIC_USER env variable!");
    let es_passwd = var("ELASTIC_PASSWORD").expect("There is not ELASTIC_PASSWORD env variable!");
    let client_addr = var("SEARCHER_ADDRESS").expect("There is not SEARCHER_ADDRESS env variable!");
    let client_port = var("SEARCHER_PORT").expect("There is not SEARCHER_PORT env variable!");
    let logger_wm = var("LOGGER_ADDR").expect("There is not LOGGER_ADDR env variable!");
    let cacher_addr = var("CACHER_HOST").expect("There is not CACHER_HOST env variable!");
    let cors_origins: String = var("CORS_ORIGIN").expect("There is not CORS_ORIGIN env variable!");
    let cacher_expire = var("CACHER_EXPIRE").expect("There is not CACHER_EXPIRE env variable!");
    let embeddings_url = var("EMBEDDINGS_URL").expect("There is not EMBEDDINGS_URL env variable!");

    let client_port =
        u16::from_str(client_port.as_str()).expect("Failed while parsing port number.");

    let cacher_expire_int =
        u64::from_str(cacher_expire.as_str()).expect("Failed while parsing cacher expire value.");

    let service = ServiceParametersBuilder::default()
        .es_host(es_host)
        .es_user(es_user)
        .es_passwd(es_passwd)
        .cacher_addr(cacher_addr)
        .service_addr(client_addr)
        .service_port(client_port)
        .cors_origin(cors_origins)
        .logger_mw_addr(logger_wm)
        .cacher_expire(cacher_expire_int)
        .embeddings_url(embeddings_url)
        .build();

    Ok(service.unwrap())
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
    web::scope("/hello").service(hello)
}

pub fn build_cluster_scope() -> Scope {
    web::scope("/cluster")
        .service(clusters::new_cluster)
        .service(clusters::delete_cluster)
        .service(clusters::all_clusters)
        .service(clusters::get_cluster)
}

pub fn build_bucket_scope() -> Scope {
    web::scope("/bucket")
        .service(buckets::new_bucket)
        .service(buckets::default_bucket)
        .service(buckets::delete_bucket)
        .service(buckets::all_buckets)
        .service(buckets::get_bucket)
        .service(buckets::get_bucket_documents)
}

pub fn build_document_scope() -> Scope {
    web::scope("/document")
        .service(documents::new_document)
        .service(documents::delete_documents)
        .service(documents::update_document)
        .service(documents::get_document)
}

pub fn build_search_scope() -> Scope {
    #[cfg(feature = "enable-chunked")]
    if cfg!(feature = "enable-chunked") {
        use crate::endpoints::searcher::{search_chunked, search_chunked_tokens};
        return web::scope("/search")
            .service(search_chunked)
            .service(search_chunked_tokens);
    }

    web::scope("/search")
        .service(searcher::search_all)
        .service(searcher::search_tokens)
}

pub fn build_similar_scope() -> Scope {
    #[cfg(feature = "enable-chunked")]
    if cfg!(feature = "enable-chunked") {
        use crate::endpoints::similarities::search_similar_chunked_docs;
        return web::scope("/similar").service(search_similar_chunked_docs);
    }

    web::scope("/similar").service(search_similar_docs)
}

pub fn build_file_scope() -> Scope {
    web::scope("/file")
        .service(loader::load_file)
        .service(loader::download_file)
}

pub fn build_pagination_scope() -> Scope {
    #[cfg(feature = "enable-chunked")]
    if cfg!(feature = "enable-chunked") {
        use crate::endpoints::paginator::next_pagination_chunked_result;
        return web::scope("/pagination")
            .service(paginator::get_pagination_ids)
            .service(paginator::delete_expired_ids)
            .service(next_pagination_chunked_result);
    }

    web::scope("/pagination")
        .service(paginator::get_pagination_ids)
        .service(paginator::delete_expired_ids)
        .service(paginator::next_pagination_result)
}
