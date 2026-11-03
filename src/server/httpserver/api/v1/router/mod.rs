pub mod document;
pub mod index;
pub mod searcher;

use doc_search_core::domain::searcher::validate_path;
use doc_search_core::shared::kernel::ResourceId;
use doc_search_core::shared::services::{IResourceManager, ResourceInfo};
use std::sync::Arc;

use crate::server::{ServerError, ServerResult};

pub(crate) async fn resolve_resource_info(
    resource_manager: Arc<dyn IResourceManager>,
    raw_resource_id: &str,
) -> ServerResult<ResourceInfo> {
    let resource_id = parse_resource_id(raw_resource_id)?;
    let info = resource_manager.get_index(&resource_id).await?;

    Ok(info)
}

fn parse_resource_id(raw: &str) -> ServerResult<ResourceId> {
    let raw = raw.trim();

    uuid::Uuid::parse_str(raw)
        .map_err(|_| ServerError::IncorrectInputForm(format!("invalid resource_id: {raw}")))?;

    Ok(ResourceId::from(raw.to_string()))
}

/// Joins the resource folder prefix with the client-provided relative path.
///
/// The resource prefix comes from resource-manager and is trusted-ish, but is
/// still validated (invalid data is reported as an internal error). The client
/// path is validated too and an invalid value is the client's fault (400).
pub(crate) fn join_path_prefix(
    resource_prefix: &str,
    client_path: Option<&str>,
) -> ServerResult<Option<String>> {
    let resource_prefix = resource_prefix.trim_matches('/');

    if !resource_prefix.is_empty() {
        validate_path(resource_prefix)
            .map_err(|err| ServerError::InternalError(format!("invalid resource prefix: {err}")))?;
    }

    let client_path = client_path.filter(|it| !it.is_empty());
    if let Some(client_path) = client_path {
        validate_path(client_path)
            .map_err(|err| ServerError::IncorrectInputForm(format!("invalid path: {err}")))?;
    }

    Ok(match (resource_prefix.is_empty(), client_path) {
        (true, None) => None,
        (true, Some(client_path)) => Some(client_path.to_string()),
        (false, None) => Some(resource_prefix.to_string()),
        (false, Some(client_path)) => Some(format!("{resource_prefix}/{client_path}")),
    })
}
