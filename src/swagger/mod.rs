pub mod examples;

use crate::errors::*;

use crate::metrics::endpoints::*;

use crate::storage::endpoints::*;
use crate::storage::forms::*;
use crate::storage::models::*;

use crate::searcher::endpoints::*;
use crate::searcher::forms::*;
use crate::searcher::models::*;

pub use utoipa::{openapi, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

const SWAGGER_TARGET_URL: &str = "/swagger/{_:.*}";
const SWAGGER_FILE_URL: &str = "/api-docs/openapi.json";

#[derive(OpenApi)]
#[openapi(
    info(
        description = "There is simple documents searcher project based on Rust and Elasticsearch technologies."
    ),
    tags(
        (
            name = "metrics",
            description = "Metrics API routes (prometheus)",
        ),
        (
            name = "search",
            description = "Search API routes",
        ),
        (
            name = "folders",
            description = "APIs to manage buckets of data storage",
        ),
        (
            name = "documents",
            description = "APIs to manage documents stored into folders",
        ),
    ),
    paths(
        hello,
        get_folder,
        get_folders,
        create_folder,
        delete_folder,
        get_document,
        get_documents,
        update_document,
        create_document,
        delete_document,
        search_fulltext,
        search_semantic,
        delete_scrolls,
        paginate_next,
    ),
    components(
        schemas(
            Successful,
            ErrorResponse,
            Folder,
            FolderType,
            CreateFolderForm,
            Document,
            DocumentPreview,
            EmbeddingsVector,
            HighlightEntity,
            DocumentType,
            FulltextParams,
            SemanticParams,
            RetrieveParams,
            Paginated<Vec<Document>>,
            DeleteScrollsForm,
            ScrollNextForm,
        ),
    ),
)]
pub struct ApiDoc;

pub fn build_swagger_service() -> SwaggerUi {
    let api_doc = ApiDoc::openapi();
    SwaggerUi::new(SWAGGER_TARGET_URL).url(SWAGGER_FILE_URL, api_doc)
}
