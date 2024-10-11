use actix_cors::Cors;
use actix_web::http::header;
use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;

#[derive(Clone, Deserialize, CopyGetters, Getters)]
pub struct CorsConfig {
    #[getset(get = "pub")]
    methods: String,
    #[getset(get = "pub")]
    allowed: String,
    #[getset(get_copy = "pub")]
    max_age: usize,
}

pub fn build_cors(config: &CorsConfig) -> Cors {
    let available_headers = vec![header::AUTHORIZATION, header::ACCEPT];
    let methods = config.methods().split(',').collect::<Vec<&str>>();

    let cors = Cors::default()
        .allowed_header(header::CONTENT_TYPE)
        .allowed_headers(available_headers)
        .allowed_methods(methods);

    let cors = match config.allowed().as_str() {
        "*" => cors.allow_any_origin(),
        allow => cors.allowed_origin(allow),
    };

    cors.max_age(config.max_age())
}
