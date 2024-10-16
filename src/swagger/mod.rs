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

#[derive(OpenApi)]
#[openapi(
    info(
        description = "There is API endpoints of DocSearch project based on Rust and Elasticsearch technologies."
    ),
    paths(
        metrics,
        get_folder,
        get_folders,
        create_folder,
        delete_folder,
        get_document,
        update_document,
        create_document,
        delete_document,
        search_fulltext,
        search_semantic,
        get_index_records,
        delete_paginate_sessions,
        paginate_next,
    ),
    components(
        schemas(
            Successful,
            ErrorResponse,
            Folder,
            CreateFolderForm,
            Document,
            DocumentPreview,
            OcrMetadata,
            Artifacts,
            GroupValue,
            HighlightEntity,
            DocumentType,
            SearchParams,
            Paginated<Vec<Document>>,
            DeletePaginationsForm,
            PaginateNextForm,
        )
    ),
    tags (
        (
            name = "DocSearcher REST API",
            description = "There is simple documents searcher project based on Rust and Elasticsearch technologies."
        )
    )
)]
pub struct ApiDoc;

pub fn build_swagger_service() -> SwaggerUi {
    let openapi = ApiDoc::openapi();
    SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", openapi.clone())
}
