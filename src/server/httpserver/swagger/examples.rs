use utoipa::ToSchema;

#[allow(dead_code)]
#[derive(utoipa::ToResponse, ToSchema)]
#[response(description = "Error form", content_type = "application/json")]
pub struct DefaultErrorForm {
    #[schema(example = 501)]
    status: u16,
    #[schema(example = "Error form")]
    message: String,
}
