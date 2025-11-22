use serde_derive::{Deserialize, Serialize};
use utoipa::IntoParams;

#[derive(Deserialize, Serialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct CreateDocumentQuery {
    pub force: Option<bool>,
}
