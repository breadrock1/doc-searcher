use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use super::models::*;
use super::router::searcher::*;
use super::router::storage::*;

use crate::application::services::server::{ServerError, Success};
use crate::infrastructure::httpserver::api::v1::swagger;

pub fn init_swagger_layer(version: &str) -> RapiDoc {
    let url_path = format!("/api/{version}/swagger");
    let config_file_path = format!("/api-docs/openapi-{version}.json");
    let swagger_app = swagger::ApiDoc::openapi();
    RapiDoc::with_openapi(config_file_path, swagger_app).path(url_path)
}

#[derive(OpenApi)]
#[openapi(
    info(
        description = "There is simple documents searcher project based on Rust and Elasticsearch technologies."
    ),
    tags(
        (
            name = "index",
            description = "CRUD operation for Index management",
        ),
        (
            name = "document",
            description = "APIs to manage documents stored into folders",
        ),
        (
            name = "search",
            description = "APIs to search Document objects",
        ),
    ),
    paths(
        get_all_indexes,
        get_index,
        create_index,
        delete_index,
        get_document,
        get_documents,
        update_document,
        store_document,
        delete_document,
        search_fulltext,
        search_semantic,
        search_hybrid,
        paginate_next,
        delete_scroll_session,
    ),
    components(
        schemas(
            CreateDocumentForm,
            CreateIndexForm,
            DocumentSchema,
            IndexSchema,
            FilterForm,
            ResultForm,
            FullTextSearchForm,
            RetrieveDocumentForm,
            SemanticSearchForm,
            HybridSearchForm,
            KnnIndexForm,
            ServerError,
            Success,
        ),
    ),
)]
struct ApiDoc;

#[allow(dead_code)]
pub trait SwaggerExample {
    type Example;

    fn example(value: Option<&str>) -> Self::Example;
}

impl SwaggerExample for Success {
    type Example = Self;

    fn example(_: Option<&str>) -> Self::Example {
        Success::default()
    }
}

impl SwaggerExample for ServerError {
    type Example = Self;

    fn example(value: Option<&str>) -> Self::Example {
        match value {
            None => ServerError::ServerUnavailable,
            Some(msg) => ServerError::InternalError(msg.to_owned()),
        }
    }
}
