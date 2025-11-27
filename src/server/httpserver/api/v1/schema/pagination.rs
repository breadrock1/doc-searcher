use derive_builder::Builder;
use doc_search_core::domain::searcher::models::Pagination;
use serde_derive::Serialize;
use utoipa::ToSchema;

use crate::server::httpserver::api::v1::schema::founded::FoundedDocumentPartSchema;
use crate::server::ServerError;

#[derive(Builder, Serialize, ToSchema)]
pub struct PaginationSchema {
    founded: Vec<FoundedDocumentPartSchema>,
    #[schema(example = "dksfsjvJHZVFDskjdbfsdfsdfdsg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scroll_id: Option<String>,
}

impl TryFrom<Pagination> for PaginationSchema {
    type Error = ServerError;

    fn try_from(paginated: Pagination) -> Result<Self, Self::Error> {
        let founded = paginated
            .founded
            .into_iter()
            .map(FoundedDocumentPartSchema::try_from)
            .filter_map(Result::ok)
            .collect();

        PaginationSchemaBuilder::default()
            .founded(founded)
            .scroll_id(paginated.scroll_id)
            .build()
            .map_err(|err| ServerError::InternalError(err.to_string()))
    }
}
