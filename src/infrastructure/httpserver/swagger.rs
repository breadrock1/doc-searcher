pub use utoipa::OpenApi;

use crate::application::dto::*;
use crate::application::services::server::error::Success;
use crate::infrastructure::httpserver::router::searcher::*;
use crate::infrastructure::httpserver::router::storage::*;

#[derive(OpenApi)]
#[openapi(
    info(
        description = "There is simple documents searcher project based on Rust and Elasticsearch technologies."
    ),
    tags(
        (
            name = "searcher",
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
            Document,
            EmbeddingChunk,
            Index,
            FullTextSearchParams,
            RetrieveDocumentParams,
            SemanticSearchParams,
            PaginateParams,
        ),
    ),
)]
pub(super) struct ApiDoc;

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
