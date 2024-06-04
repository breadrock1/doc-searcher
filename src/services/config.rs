use crate::services::init::build_env_logger;

use derive_builder::Builder;
use derive_getters::Getters;

#[derive(Builder, Getters)]
pub struct ServiceConfig {
    service_host: String,
    service_port: u16,
    elastic_host: String,
    elastic_user: String,
    elastic_passwd: String,
    cacher_host: String,
    cacher_passwd: String,
    cacher_expire: u64,
    llm_host: String,
    watcher_host: String,
    logger_host: String,
    cors_origin: String,
    workers_num: usize,
}

pub fn init_service_config() -> Result<ServiceConfig, anyhow::Error> {
    #[cfg(feature = "enable-dotenv")]
    {
        use dotenv::dotenv;
        dotenv().ok();
    }

    build_env_logger();

    let service = ServiceConfigBuilder::default()
        .service_host(extract_env_value("SERVICE_HOST"))
        .service_port(extract_int_env_value::<u16>("SERVICE_PORT"))
        .elastic_host(extract_env_value("ELASTIC_SERVICE_HOST"))
        .elastic_user(extract_env_value("ELASTIC_SERVICE_USERNAME"))
        .elastic_passwd(extract_env_value("ELASTIC_SERVICE_PASSWORD"))
        .cacher_host(extract_env_value("CACHER_SERVICE_HOST"))
        .cacher_passwd(extract_env_value("CACHER_SERVICE_PASSWORD"))
        .cacher_expire(extract_int_env_value::<u64>("CACHER_SERVICE_EXPIRE"))
        .llm_host(extract_env_value("LLM_SERVICE_HOST"))
        .watcher_host(extract_env_value("WATCHER_SERVICE_HOST"))
        .logger_host(extract_env_value("LOGGER_SERVICE_HOST"))
        .cors_origin(extract_env_value("ALLOWED_CORS"))
        .workers_num(extract_int_env_value::<usize>("WORKERS_NUMBER"))
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
