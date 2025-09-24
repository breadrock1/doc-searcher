use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use super::models::*;
use super::router::searcher::*;
use super::router::storage::*;

use crate::application::services::server::{ServerError, Success};
use crate::infrastructure::httpserver::api::v1::swagger;

pub fn init_swagger_layer(version: &str) -> SwaggerUi {
    let url_path = format!("/api/{version}/swagger");
    let config_file_path = format!("/api-docs/openapi-{version}.json");
    let swagger_app = swagger::ApiDoc::openapi();
    SwaggerUi::new(url_path).url(config_file_path, swagger_app)
}

const DESCRIPTION: &str = include_str!("../../../../../docs/swagger/swagger-ui.description");

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Doc-Search",
        description = DESCRIPTION,
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
    servers(
        (url = "/api/v1", description = "Stable API version"),
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

impl SwaggerExample for ServerError {
    type Example = Self;

    fn example(value: Option<&str>) -> Self::Example {
        match value {
            None => ServerError::ServerUnavailable,
            Some(msg) => ServerError::InternalError(msg.to_owned()),
        }
    }
}

#[allow(dead_code)]
#[derive(utoipa::ToResponse, ToSchema)]
#[response(description = "Error form", content_type = "application/json")]
pub struct DefaultErrorForm {
    #[schema(example = 501)]
    status: u16,
    #[schema(example = "Error form")]
    message: String,
}
