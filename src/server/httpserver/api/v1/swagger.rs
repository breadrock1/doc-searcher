use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use crate::server::ServerError;
use crate::server::Success;

use super::form::*;
use super::router::document::*;
use super::router::index::*;
use super::router::searcher::*;
use super::schema::*;

use crate::server::httpserver::api::v1::swagger;

pub fn init_swagger_layer(version: &str) -> SwaggerUi {
    let url_path = format!("/api/{version}/swagger");
    let config_file_path = format!("/api-docs/openapi-{version}.json");
    let swagger_app = swagger::ApiDoc::openapi();
    SwaggerUi::new(url_path).url(config_file_path, swagger_app)
}

const DESCRIPTION: &str = include_str!("../../../../../docs/swagger/swagger-ui");

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
        get_document_parts,
        get_index_documents,
        store_document,
        delete_document,
        search_fulltext,
        search_semantic,
        search_hybrid,
        paginate_next,
    ),
    components(
        schemas(
            CreateDocumentForm,
            CreateIndexForm,
            DocumentPartSchema,
            IndexSchema,
            FilterForm,
            ResultForm,
            ShortResultForm,
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
#[derive(utoipa::ToResponse, ToSchema)]
#[response(description = "Error form", content_type = "application/json")]
pub struct DefaultErrorForm {
    #[schema(example = 501)]
    status: u16,
    #[schema(example = "Error form")]
    message: String,
}
