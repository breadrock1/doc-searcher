pub mod client;
pub mod context;

use crate::service::own_engine::context::OtherContext;
use std::error::Error;

pub type ClientBuildResult = Result<OtherContext, Box<dyn Error>>;

pub fn build_own_client(_es_host: &str, _es_user: &str, _es_passwd: &str) -> ClientBuildResult {
    Ok(OtherContext::_new(String::from("Own client")))
}
