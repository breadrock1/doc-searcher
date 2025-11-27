use doc_search_core::domain::storage::models::{
    CreateIndexParams, CreateIndexParamsBuilder, KnnIndexParams, KnnIndexParamsBuilder,
};
use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[allow(unused_imports)]
use serde_json::json;

use crate::server::ServerError;

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct CreateIndexForm {
    #[schema(example = "test-folder")]
    id: String,
    #[schema(value_type = Option<KnnIndexForm>, example = KnnIndexForm)]
    knn: Option<KnnIndexForm>,
}

impl TryFrom<CreateIndexForm> for CreateIndexParams {
    type Error = ServerError;

    fn try_from(form: CreateIndexForm) -> Result<Self, Self::Error> {
        let knn_params = form
            .knn
            .map(|it| KnnIndexForm::try_into(it).ok())
            .unwrap_or_default();
        CreateIndexParamsBuilder::default()
            .id(form.id)
            .knn(knn_params)
            .build()
            .map_err(|err| ServerError::IncorrectInputForm(err.to_string()))
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct KnnIndexForm {
    #[schema(example = 768)]
    knn_dimension: u32,
    #[schema(example = 50)]
    token_limit: u32,
    #[schema(example = 0.2)]
    overlap_rate: f32,
}

impl TryFrom<KnnIndexForm> for KnnIndexParams {
    type Error = ServerError;

    fn try_from(form: KnnIndexForm) -> Result<Self, Self::Error> {
        KnnIndexParamsBuilder::default()
            .knn_dimension(form.knn_dimension)
            .token_limit(form.token_limit)
            .overlap_rate(form.overlap_rate)
            .build()
            .map_err(|err| ServerError::IncorrectInputForm(err.to_string()))
    }
}
