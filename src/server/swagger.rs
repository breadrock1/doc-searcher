use crate::errors::*;

use crate::engine::form::*;
use crate::engine::model::*;
use crate::server::router::searcher::*;
use crate::server::router::storage::*;

pub use utoipa::OpenApi;

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
            DeleteScrollsForm,
            ScrollNextForm,
        ),
    ),
)]
pub (super) struct ApiDoc;

pub trait SwaggerExample {
    type Example;

    fn example(value: Option<&str>) -> Self::Example;
}

impl SwaggerExample for Successful {
    type Example = Self;

    fn example(value: Option<&str>) -> Self::Example {
        let msg = value.unwrap_or("Done");
        Successful::new(200, msg)
    }
}

impl SwaggerExample for ErrorResponse {
    type Example = Self;

    fn example(value: Option<&str>) -> Self::Example {
        let msg = value.unwrap_or("bad client request");
        ErrorResponse::new(400, "Bad request", msg)
    }
}
