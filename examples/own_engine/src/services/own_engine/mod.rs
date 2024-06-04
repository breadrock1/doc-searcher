pub mod clusters;
pub(crate) mod context;
pub mod documents;
pub mod folders;
pub mod paginator;
pub mod searcher;
pub mod watcher;

use crate::services::init::ServiceParameters;
use crate::services::own_engine::context::OtherContext;

pub fn build_own_service(_sv_params: &ServiceParameters) -> Result<OtherContext, anyhow::Error> {
    Ok(OtherContext::new(String::from("Own client")))
}
