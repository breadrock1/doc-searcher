use elquery::exclude::ExcludeFields;
use elquery::filter::must_filter::BoolMustFilter;
use elquery::r#match::{BoolQuery, BoolQueryType};
use elquery::search::multi_match_query::BoolMultiMatchQuery;
use elquery::search::should_query::{BoolShouldQuery, MatchItemQuery, MatchItemType};
use elquery::sort::{SortItem, SortItemFormat, SortItemOrder, SortQuery};
use elquery::CommonQuery;
use serde_json::Value;

use crate::engine::error::StorageResult;
use crate::engine::form::RetrieveParams;
use crate::engine::model::{Document, DocumentVectors, DocumentsTrait, InfoFolder};

#[async_trait::async_trait]
pub trait Retrieve<'de, T>
where
    T: DocumentsTrait + serde::Serialize + serde::Deserialize<'de>,
{
    type Params;

    async fn build_retrieve_query(params: &Self::Params) -> Value;

    fn extract_from_response(value: &Value) -> StorageResult<T>;
}

#[async_trait::async_trait]
impl Retrieve<'_, Document> for Document {
    type Params = RetrieveParams;

    async fn build_retrieve_query(params: &Self::Params) -> Value {
        let (doc_size_from, doc_size_to) = params.document_size();
        let (doc_cr_from, doc_cr_to) = params.document_dates();
        let doc_ext = params.document_extension().clone();
        let query = params.query();

        let must_filter = BoolMustFilter::default()
            .with_range("document_created", doc_cr_from, doc_cr_to)
            .with_range("document_size", doc_size_from, doc_size_to)
            .with_term("document_extension", doc_ext)
            .build();

        let bool_query = match query.is_none() {
            true => BoolQuery::default()
                .with_match_all(BoolQueryType::Must)
                .with_filter(must_filter)
                .build(),
            false => {
                let fields = vec!["document_name".to_string(), "document_path".to_string()];

                let item_query = MatchItemQuery::builder()
                    .query(query.clone().unwrap_or_default())
                    .item_type(Some(MatchItemType::PhrasePrefix))
                    .minimum_should_match("50%".to_string())
                    .fields(fields)
                    .build()
                    .unwrap();

                let multi_match_query = BoolMultiMatchQuery::default().set_item(item_query).build();

                let should_query = BoolShouldQuery::default()
                    .append_item(multi_match_query)
                    .build();

                BoolQuery::default()
                    .with_query(should_query, BoolQueryType::Should)
                    .with_filter(must_filter)
                    .build()
            }
        };

        let sort_item = SortItem::default()
            .with_order(SortItemOrder::Desc)
            .with_format(SortItemFormat::StrictDateOptionalTimeNanos)
            .build();

        let sort_query = SortQuery::default()
            .with_must_field("document_created", sort_item)
            .build();

        let sort_queries = vec![serde_json::to_value(sort_query).unwrap()];

        let exclude_query = ExcludeFields::default().with_fields(vec!["embeddings".to_string()]);

        let query = CommonQuery::builder()
            .query(bool_query)
            .min_score(None)
            .highlight(None)
            .sort(Some(sort_queries))
            ._source(Some(exclude_query))
            .build()
            .unwrap();

        serde_json::to_value(query).unwrap()
    }

    fn extract_from_response(value: &Value) -> StorageResult<Document> {
        let value = &value[&"_source"];
        let document = serde_json::from_value::<Document>(value.to_owned())?;
        Ok(document)
    }
}

#[async_trait::async_trait]
impl Retrieve<'_, DocumentVectors> for DocumentVectors {
    type Params = RetrieveParams;

    async fn build_retrieve_query(_params: &Self::Params) -> Value {
        let match_all_query = BoolQuery::default()
            .with_match_all(BoolQueryType::Must)
            .build();

        let query = CommonQuery::builder()
            .query(match_all_query)
            .sort(None)
            .min_score(None)
            ._source(None)
            .highlight(None)
            .build()
            .unwrap();

        serde_json::to_value(query).unwrap()
    }

    fn extract_from_response(value: &Value) -> StorageResult<DocumentVectors> {
        let value = &value[&"_source"];
        let document = serde_json::from_value::<DocumentVectors>(value.to_owned())?;
        Ok(document)
    }
}

#[async_trait::async_trait]
impl Retrieve<'_, InfoFolder> for InfoFolder {
    type Params = RetrieveParams;

    async fn build_retrieve_query(params: &Self::Params) -> Value {
        let must_filter = BoolMustFilter::default();
        let must_filter = match params.is_show_all() {
            true => must_filter,
            false => must_filter.with_term("is_system", Some("false")),
        }
        .with_exists(Some("folder_type"))
        .build();

        let match_all_query = BoolQuery::default()
            .with_match_all(BoolQueryType::Must)
            .with_filter(must_filter)
            .build();

        let query = CommonQuery::builder()
            .query(match_all_query)
            .sort(None)
            .min_score(None)
            ._source(None)
            .highlight(None)
            .build()
            .unwrap();

        serde_json::to_value(query).unwrap()
    }

    fn extract_from_response(value: &Value) -> StorageResult<InfoFolder> {
        let value = &value[&"_source"];
        let document = serde_json::from_value::<InfoFolder>(value.to_owned())?;
        Ok(document)
    }
}
