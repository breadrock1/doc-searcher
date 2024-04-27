use crate::endpoints::buckets::__path_all_buckets;
use crate::endpoints::buckets::__path_default_bucket;
use crate::endpoints::buckets::__path_delete_bucket;
use crate::endpoints::buckets::__path_get_bucket;
use crate::endpoints::buckets::__path_new_bucket;
use crate::endpoints::buckets::__path_get_bucket_documents;

use crate::endpoints::clusters::__path_all_clusters;
use crate::endpoints::clusters::__path_delete_cluster;
use crate::endpoints::clusters::__path_get_cluster;
use crate::endpoints::clusters::__path_new_cluster;

use crate::endpoints::documents::__path_delete_documents;
use crate::endpoints::documents::__path_get_document;
use crate::endpoints::documents::__path_new_document;
use crate::endpoints::documents::__path_update_document;

use crate::endpoints::hello::__path_hello;

use crate::endpoints::loader::__path_download_file;
use crate::endpoints::loader::__path_load_file;

use crate::endpoints::paginator::__path_delete_expired_ids;
use crate::endpoints::paginator::__path_get_pagination_ids;
use crate::endpoints::paginator::__path_next_pagination_result;

use crate::endpoints::searcher::__path_search_all;

use crate::endpoints::similarities::__path_search_similar_docs;

use crate::errors::*;

use wrappers::bucket::*;
use wrappers::cluster::*;
use wrappers::document::*;
use wrappers::scroll::*;
use wrappers::file_form::*;
use wrappers::search_params::*;

pub use utoipa::{openapi, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        description = "There is API endpoints of DocSearch project based on Rust and Elasticsearch technologies."
    ),
    paths(
        hello,
        all_buckets,
        default_bucket,
        delete_bucket,
        get_bucket,
        new_bucket,
        get_bucket_documents,
        all_clusters,
        delete_cluster,
        get_cluster,
        new_cluster,
        update_document,
        get_document,
        new_document,
        delete_documents,
        load_file,
        download_file,
        get_pagination_ids,
        delete_expired_ids,
        next_pagination_result,
        search_all,
        search_similar_docs,
    ),
    components(
        schemas(
            ErrorResponse,
            SuccessfulResponse,
            Bucket,
            BucketForm,
            Cluster,
            ClusterForm,
            Document,
            HighlightEntity,
            OcrMetadata,
            Artifacts,
            LoadFileForm,
            SearchParams,
            PagintatedResult<Vec<Document>>,
            NextScroll,
            AllScrolls,
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
    SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", openapi.clone())
}
