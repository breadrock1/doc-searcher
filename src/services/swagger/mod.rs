use crate::endpoints::clusters::*;
use crate::endpoints::documents::*;
use crate::endpoints::folders::*;
use crate::endpoints::hello::*;
use crate::endpoints::paginator::*;
use crate::endpoints::searcher::*;
use crate::endpoints::watcher::*;

use crate::errors::*;

use crate::forms::cluster::*;
use crate::forms::document::*;
use crate::forms::folder::*;
use crate::forms::s_params::*;
use crate::forms::scroll::*;

pub use utoipa::{openapi, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        description = "There is API endpoints of DocSearch project based on Rust and Elasticsearch technologies."
    ),
    paths(
        hello,
        all_clusters,
        create_cluster,
        get_cluster,
        delete_cluster,
        all_folders,
        create_folder,
        get_folder,
        delete_folder,
        get_folder_documents,
        create_document,
        delete_documents,
        get_document,
        update_document,
        upload_files,
        get_pagination_ids,
        delete_expired_ids,
        next_pagination_result,
        search_all,
        search_similar_docs,
        analyse_documents,
    ),
    components(
        schemas(
            ErrorResponse,
            SuccessfulResponse,
            Folder,
            FolderForm,
            Cluster,
            ClusterForm,
            Document,
            HighlightEntity,
            OcrMetadata,
            Artifacts,
            SearchParams,
            Paginated<Vec<Document>>,
            NextScrollForm,
            AllScrollsForm,
            HighlightEntity,
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

pub fn create_service(openapi: &openapi::OpenApi) -> SwaggerUi {
    SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", openapi.clone())
}
