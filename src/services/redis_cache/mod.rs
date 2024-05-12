use crate::services::cacher::CacherClient;
use crate::services::init::ServiceParameters;
use crate::services::redis_cache::client::RedisService;

pub mod client;
pub mod values;

pub fn build_redis_service(
    sv_params: &ServiceParameters,
) -> Result<CacherClient<RedisService>, anyhow::Error> {
    let address = sv_params.cacher_service_host();
    let expire = sv_params.cacher_expire();
    let redis_client = RedisService::new(address, *expire);
    Ok(CacherClient::new(redis_client))
}
