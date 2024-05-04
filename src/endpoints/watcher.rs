use crate::endpoints::SearcherData;
use crate::errors::{ErrorResponse, JsonResponse, PaginateJsonResponse};

use wrappers::TestExample;
use wrappers::document::{AnalyseDocumentsForm, DocumentPreview};
use wrappers::search_params::SearchParams;
use wrappers::scroll::PaginatedResult;

use actix_web::{web, post};

#[utoipa::path(
    post,
    path = "/watcher/analyse",
    tag = "Watcher",
    request_body(
        content = AnalyseDocumentsForm,
        example = json!(AnalyseDocumentsForm::test_example(None)),
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
    form: web::Json<AnalyseDocumentsForm>
) -> JsonResponse<Vec<DocumentPreview>> {
    let client = cxt.get_ref();
    let document_ids = form.0.document_ids;
    client.launch_watcher_analysis(document_ids.as_slice()).await
}

#[utoipa::path(
    post,
    path = "/watcher/unrecognized",
    tag = "Watcher",
    request_body(
        content = SearchParams,
        example = json!(SearchParams::test_example(Some("Transport"))),
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
#[post("/unrecognized")]
async fn get_folder_documents2(
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
