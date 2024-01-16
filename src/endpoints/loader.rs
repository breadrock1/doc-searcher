use crate::endpoints::ContextData;
use crate::wrappers::file_form::LoadFileForm;

use actix_web::{post, web, HttpResponse};

#[post("/loader/load")]
async fn load_file(cxt: ContextData, form: web::Json<LoadFileForm>) -> HttpResponse {
    let client = cxt.get_ref();
    let file_path = form.get_path();
    let bucket_name = form.get_bucket();
    client.load_file_to_bucket(bucket_name, file_path).await
}

#[cfg(test)]
mod loader_endpoints {
    use crate::searcher::own_engine::context::OtherContext;
    use crate::searcher::service_client::ServiceClient;

    use actix_web::test;

    #[test]
    async fn test_load_file() {
        let file_path = "src/features/loader/resources/file_1.txt";
        let other_context = OtherContext::_new("test".to_string());
        let response = other_context
            .load_file_to_bucket("test_bucket", file_path)
            .await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }
}
