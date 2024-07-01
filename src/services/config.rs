use crate::services::init::build_env_logger;

use derive_builder::Builder;

#[derive(Builder)]
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
    logger_host: String,
    cors_origin: String,
    workers_num: usize,
}

impl ServiceConfig {
    pub fn get_service_host(&self) -> &str {
        self.service_host.as_str()
    }
    pub fn get_service_port(&self) -> u16 {
        self.service_port
    }
    pub fn get_elastic_host(&self) -> &str {
        self.elastic_host.as_str()
    }
    pub fn get_elastic_user(&self) -> &str {
        self.elastic_user.as_str()
    }
    pub fn get_elastic_passwd(&self) -> &str {
        self.elastic_passwd.as_str()
    }
    pub fn get_cacher_addr(&self) -> &str {
        self.cacher_host.as_str()
    }
    pub fn get_cacher_passwd(&self) -> &str {
        self.cacher_passwd.as_str()
    }
    pub fn get_cacher_expire(&self) -> u64 {
        self.cacher_expire
    }
    pub fn get_logger_addr(&self) -> &str {
        self.logger_host.as_str()
    }
    pub fn get_llm_addr(&self) -> &str {
        self.llm_host.as_str()
    }
    pub fn get_cors(&self) -> &str {
        self.cors_origin.as_str()
    }
    pub fn get_workers_num(&self) -> usize {
        self.workers_num
    }
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
        .cacher_expire(extract_int_env_value::<u64>("CACHER_VALUES_EXPIRE"))
        .llm_host(extract_env_value("LLM_SERVICE_HOST"))
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
