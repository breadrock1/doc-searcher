use crate::endpoints::CacherData;
use crate::errors::{ErrorResponse, PaginateResponse};
use crate::forms::documents::document::Document;
use crate::forms::pagination::Paginated;
use crate::forms::s_params::SearchParams;
use crate::forms::TestExample;
use crate::services::cacher::CacherService;
use crate::services::redis_cache::values::*;
use crate::services::service::SearcherService;

use actix_web::{post, web};

type Context = web::Data<Box<dyn SearcherService>>;

#[utoipa::path(
    post,
    path = "/search/",
    tag = "Search",
    request_body(
        content = SearchParams,
        example = json!(SearchParams::test_example(Some("Ocean Carrier")))
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Paginated<Vec<Document>>,
            example = json!(Paginated::<Vec<Document>>::new_with_id(
                vec![Document::test_example(None)],
                "DXF1ZXJ5QW5kRmV0Y2gBAD4WYm9laVYtZndUQlNsdDcwakFMNjU1QQ==".to_string(),
            ))
        ),
        (
            status = 400,
            description = "Failed while searching documents",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while searching documents".to_string(),
            })
        ),
    )
)]
#[post("/")]
async fn search_all(
    cxt: Context,
    cacher: CacherData,
    form: web::Json<SearchParams>,
) -> PaginateResponse<Vec<Document>> {
    let client = cxt.get_ref();
    let search_form = form.0;

    #[cfg(feature = "disable-caching")]
    if cfg!(feature = "disable-caching") {
        return client.search(&search_form).await;
    }

    let cacher_params = CacherSearchParams::from(&search_form);
    match cacher
        .service
        .load::<CacherSearchParams, CacherDocuments>(cacher_params)
        .await
    {
        None => client.search(&search_form).await,
        Some(documents) => {
            let cacher_params = CacherSearchParams::from(&search_form);
            let docs = cacher
                .service
                .insert::<CacherSearchParams, CacherDocuments>(cacher_params, documents)
                .await;

            let docs = Vec::from(docs);
            let scroll = Paginated::new(docs);
            Ok(web::Json(scroll))
        }
    }
}

#[utoipa::path(
    post,
    path = "/search/tokens",
    tag = "Search",
    request_body(
        content = SearchParams,
        example = json!(SearchParams::test_example(Some("Ocean Carrier")))
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = [Document],
            example = json!(Paginated::<Vec<Document>>::new_with_id(
                vec![Document::test_example(None)],
                "DXF1ZXJ5QW5kRmV0Y2gBAD4WYm9laVYtZndUQlNsdDcwakFMNjU1QQ==".to_string(),
            ))
        ),
        (
            status = 400,
            description = "Failed while searching tokens",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while searching tokens".to_string(),
            })
        ),
    )
)]
#[post("/tokens")]
async fn search_tokens(
    cxt: Context,
    cacher: CacherData,
    form: web::Json<SearchParams>,
) -> PaginateResponse<Vec<Document>> {
    let client = cxt.get_ref();
    let search_form = form.0;

    #[cfg(feature = "disable-caching")]
    if cfg!(feature = "disable-caching") {
        return client.search_tokens(&search_form).await;
    }

    let cacher_params = CacherSearchParams::from(&search_form);
    match cacher
        .service
        .load::<CacherSearchParams, CacherDocuments>(cacher_params)
        .await
    {
        None => client.search_tokens(&search_form).await,
        Some(documents) => {
            let cacher_params = CacherSearchParams::from(&search_form);
            let docs = cacher
                .service
                .insert::<CacherSearchParams, CacherDocuments>(cacher_params, documents)
                .await;

            let docs = Vec::from(docs);
            let scroll = Paginated::new(docs);
            Ok(web::Json(scroll))
        }
    }
}

#[utoipa::path(
    post,
    path = "/search/similar",
    tag = "Search",
    request_body(
        content = SearchParams,
        example = json!(SearchParams::test_example(Some("12:JOGnP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y")))
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = [Document],
            example = json!(Paginated::<Vec<Document>>::new_with_id(
                vec![Document::test_example(None)],
                "DXF1ZXJ5QW5kRmV0Y2gBAD4WYm9laVYtZndUQlNsdDcwakFMNjU1QQ==".to_string(),
            ))
        ),
        (
            status = 400,
            description = "Failed while searching similar documents",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while searching similar documents".to_string(),
            })
        ),
    )
)]
#[post("/similar")]
async fn search_similar_docs(
    cxt: Context,
    cacher: CacherData,
    form: web::Json<SearchParams>,
) -> PaginateResponse<Vec<Document>> {
    let client = cxt.get_ref();
    let search_form = form.0;

    #[cfg(feature = "disable-caching")]
    if cfg!(feature = "disable-caching") {
        return client.similarity(&search_form).await;
    }

    let cacher_params = CacherSearchParams::from(&search_form);
    match cacher
        .service
        .load::<CacherSearchParams, CacherDocuments>(cacher_params)
        .await
    {
        None => client.similarity(&search_form).await,
        Some(documents) => {
            let cacher_params = CacherSearchParams::from(&search_form);
            let docs = cacher
                .service
                .insert::<CacherSearchParams, CacherDocuments>(cacher_params, documents)
                .await;

            let docs = Vec::from(docs);
            let scroll = Paginated::new(docs);
            Ok(web::Json(scroll))
        }
    }
}
