mod clusters;
pub(crate) mod context;
mod documents;
mod folders;
mod paginator;
mod searcher;
mod watcher;

use crate::services::init::ServiceParameters;
use crate::services::own_engine::context::OtherContext;

pub fn build_own_service(_sv_params: &ServiceParameters) -> Result<OtherContext, anyhow::Error> {
    Ok(OtherContext::new(String::from("Own client")))
}
