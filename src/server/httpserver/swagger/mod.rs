mod examples;
pub use examples::DefaultErrorForm;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::server::ServerError;
use crate::server::Success;

use crate::server::httpserver::api::v1::form::*;
use crate::server::httpserver::api::v1::router::document::*;
use crate::server::httpserver::api::v1::router::index::*;
use crate::server::httpserver::api::v1::router::searcher::*;
use crate::server::httpserver::api::v1::schema::*;

const SWAGGER_URL_PATH: &str = "/api/swagger";
const SWAGGER_CONFIG_PATH: &str = "/api-docs/openapi.json";
const PROJECT_DESCRIPTION: &str = include_str!("./descriptions/project");

pub fn init_swagger_layer() -> SwaggerUi {
    let swagger_app = ApiDoc::openapi();
    SwaggerUi::new(SWAGGER_URL_PATH).url(SWAGGER_CONFIG_PATH, swagger_app)
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Doc-Search",
        description = PROJECT_DESCRIPTION,
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
        // v1 routes
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
            UpdateDocumentForm,
            CreateIndexForm,
            KnnIndexForm,
            DocumentPartSchema,
            IndexSchema,
            FilterForm,
            ResultForm,
            ShortResultForm,
            FullTextSearchForm,
            RetrieveDocumentForm,
            SemanticSearchForm,
            HybridSearchForm,
            ServerError,
            Success,
        ),
    ),
)]
struct ApiDoc;
