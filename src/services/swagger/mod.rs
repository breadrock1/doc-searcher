use crate::endpoints::clusters::*;
use crate::endpoints::documents::*;
use crate::endpoints::folders::*;
use crate::endpoints::hello::*;
use crate::endpoints::paginator::*;
use crate::endpoints::searcher::*;

use crate::errors::*;

use crate::forms::clusters::cluster::*;
use crate::forms::clusters::forms::*;
use crate::forms::documents::document::*;
use crate::forms::documents::forms::*;
use crate::forms::documents::preview::*;
use crate::forms::documents::metadata::*;
use crate::forms::folders::folder::*;
use crate::forms::folders::forms::*;
use crate::forms::pagination::forms::*;
use crate::forms::pagination::pagination::Paginated;
use crate::forms::searcher::s_params::*;


pub use utoipa::{openapi, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        description = "There is API endpoints of DocSearch project based on Rust and Elasticsearch technologies."
    ),
    paths(
        hello,
        get_cluster,
        get_clusters,
        create_cluster,
        delete_cluster,
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
            Cluster,
            CreateClusterForm,
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
