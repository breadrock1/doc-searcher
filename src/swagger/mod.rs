use crate::endpoints::buckets::__path_all_buckets;
use crate::endpoints::buckets::__path_default_bucket;
use crate::endpoints::buckets::__path_delete_bucket;
use crate::endpoints::buckets::__path_get_bucket;
use crate::endpoints::buckets::__path_new_bucket;

use crate::endpoints::clusters::__path_all_clusters;
use crate::endpoints::clusters::__path_delete_cluster;
use crate::endpoints::clusters::__path_get_cluster;
use crate::endpoints::clusters::__path_new_cluster;

use crate::endpoints::documents::__path_delete_document;
use crate::endpoints::documents::__path_get_document;
use crate::endpoints::documents::__path_new_document;
use crate::endpoints::documents::__path_update_document;

use crate::endpoints::hello::__path_hello;

use crate::endpoints::loader::__path_download_file;
use crate::endpoints::loader::__path_load_file;

use crate::endpoints::searcher::__path_search_all;
use crate::endpoints::searcher::__path_search_target;

use crate::endpoints::similarities::__path_search_similar_docs;
use crate::endpoints::similarities::__path_search_similar_docs_target;

use crate::errors::*;
use wrappers::bucket::*;
use wrappers::cluster::*;
use wrappers::document::*;
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
        all_clusters,
        delete_cluster,
        get_cluster,
        new_cluster,
        update_document,
        get_document,
        new_document,
        delete_document,
        load_file,
        download_file,
        search_all,
        search_target,
        search_similar_docs,
        search_similar_docs_target,
    ),
    components(
        schemas(
            ErrorResponse,
            SuccessfulResponse,
            Bucket,
            Cluster,
            Document,
            SearchParams,
            HighlightEntity,
        )
    ),
    tags ((
        name = "DocSearcher REST API",
        description = "There is simple documents searcher project based on Rust and Elasticsearch technologies."
    ))
)]
pub struct ApiDoc;

pub fn create_service(openapi: &utoipa::openapi::OpenApi) -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone())
}
