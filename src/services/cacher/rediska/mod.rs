use crate::services::cacher::rediska::client::RedisService;
use crate::services::cacher::service::CacherClient;
use crate::services::config::ServiceConfig;

pub mod client;
pub mod values;

pub type InitCacherResult = Result<CacherClient<RedisService>, anyhow::Error>;

pub fn build_cacher_service(s_config: &ServiceConfig) -> InitCacherResult {
    let address = s_config.get_cacher_addr();
    let expire = s_config.get_cacher_expire();
    let redis_client = RedisService::new(address, expire);
    Ok(CacherClient::new(redis_client))
}
