use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::application::dto::*;
use crate::application::services::server::{ServerError, Success};
use crate::infrastructure::httpserver::router::searcher::*;
use crate::infrastructure::httpserver::router::storage::*;
use crate::infrastructure::httpserver::{swagger, SWAGGER_CONFIG_FILE};

pub fn init_swagger_layer() -> RapiDoc {
    let swagger_app = swagger::ApiDoc::openapi();
    RapiDoc::with_openapi(SWAGGER_CONFIG_FILE, swagger_app).path("/rapidoc")
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
            name = "documents",
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
        paginate_next,
        delete_scroll_session,
    ),
    components(
        schemas(
            Document,
            Index,
            FilterParams,
            ResultParams,
            FullTextSearchParams,
            RetrieveDocumentParams,
            SemanticSearchParams,
            PaginateParams,
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
