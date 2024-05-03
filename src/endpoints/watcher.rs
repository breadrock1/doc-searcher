use crate::endpoints::SearcherData;
use crate::errors::{ErrorResponse, JsonResponse, PaginateJsonResponse};

use wrappers::document::DocumentPreview;
use wrappers::TestExample;

use actix_web::{web, post};
use wrappers::scroll::PaginatedResult;
use wrappers::search_params::SearchParams;

#[utoipa::path(
    post,
    path = "/watcher/analyse",
    tag = "Watcher",
    request_body(
        content = Vec<String>,
        example = json!(vec!["<document-id>"]),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = SuccessfulResponse,
            example = json!(vec![DocumentPreview::test_example(None)])
        ),
            (
            status = 400,
            description = "Failed while analysing documents",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while analysing documents".to_string(),
            })
        ),
    )
)]
#[post("/analyse")]
async fn analyse_documents(
    cxt: SearcherData, 
    form: web::Json<Vec<String>>
) -> JsonResponse<Vec<DocumentPreview>> {
    // TODO: Store DocumentPreview to history
    let client = cxt.get_ref();
    client.launch_watcher_analysis(form.as_slice()).await
}

#[post("/unrecognized")]
async fn get_folder_documents(
    cxt: SearcherData,
    form: web::Json<SearchParams>,
) -> PaginateJsonResponse<Vec<DocumentPreview>> {
    let client = cxt.get_ref();
    let search_form = form.0;
    match client
        .get_folder_documents("unrecognized", Some(search_form))
        .await
    {
        Err(err) => Err(err),
        Ok(value) => {
            let scroll_id = value.get_scroll_id().cloned();
            let preview = value
                .get_founded()
                .to_owned()
                .into_iter()
                .map(DocumentPreview::from)
                .collect();

            Ok(web::Json(PaginatedResult::new_with_opt_id(
                preview, scroll_id,
            )))
        }
    }
}
