use crate::errors::{ErrorResponse, Successful};
use crate::searcher::forms::{DeleteScrollsForm, ScrollNextForm};
use crate::searcher::forms::{FulltextParams, SemanticParams};
use crate::searcher::models::Paginated;
use crate::storage::forms::{CreateFolderForm, RetrieveParams};
use crate::storage::models::{Document, DocumentBuilder, DocumentPreview, DocumentVectors};
use crate::storage::models::{DocumentsTrait, EmbeddingsVector, Folder, FolderType};

use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};

const TEST_CLUSTER_ID: &str = "d93df49fa6ff";

const TEST_FOLDER_ID: &str = "test-folder";
const TEST_FOLDER_UUID: &str = "fDdHOrwMSESM9OlhLsrMWQ";
const TEST_FOLDER_NAME: &str = "Test Folder";
const TEST_FOLDER_PATH: &str = "./indexer/test-folder";

const DOCUMENT_ID: &str = "98ac9896be35f47fb8442580cd9839b4";
const DOCUMENT_SSDEEP_HASH: &str = "12:JOGnP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y";
const DOCUMENT_NAME: &str = "test-document.txt";
const DOCUMENT_PATH: &str = "./indexer/test-folder/test_document.txt";
const DOCUMENT_TYPE: &str = "document";
const DOCUMENT_CONTENT: &str = "The Ocean Carrier has been signed.";
const DOCUMENT_EXTENSION: &str = "txt";

const SEARCH_QUERY: &str = "Hello world!";
const SEARCH_SCROLL_LIFETIME: &str = "20m";
const SEARCH_CREATED_TO: &str = "2024-04-26T11:14:55Z";
const SEARCH_CREATED_FROM: &str = "2024-04-02T13:51:32Z";

const PAGINATE_HASH_ID: &str = "DXF1ZXJ5QW5kRmV0Y2gBAD4WYm9laVYtZndUQlNsdDcwakFMNjU1QQ==";

pub trait TestExample<T> {
    fn test_example(value: Option<&str>) -> T;
}

impl TestExample<Successful> for Successful {
    fn test_example(value: Option<&str>) -> Successful {
        let msg = value.unwrap_or("Done");
        Successful::new(200, msg)
    }
}

impl TestExample<ErrorResponse> for ErrorResponse {
    fn test_example(value: Option<&str>) -> ErrorResponse {
        let msg = value.unwrap_or("bad client request");
        ErrorResponse::new(400, "Bad request", msg)
    }
}

impl TestExample<CreateFolderForm> for CreateFolderForm {
    fn test_example(_value: Option<&str>) -> CreateFolderForm {
        CreateFolderForm::builder()
            .folder_id(TEST_FOLDER_ID.to_string())
            .folder_name(TEST_FOLDER_NAME.to_string())
            .folder_type(FolderType::Document)
            .create_into_watcher(false)
            .location(TEST_FOLDER_PATH.to_string())
            .user_id("admin".to_string())
            .is_system(false)
            .build()
            .unwrap()
    }
}

impl TestExample<Folder> for Folder {
    fn test_example(_value: Option<&str>) -> Folder {
        Folder::builder()
            .health("yellow".to_string())
            .status("open".to_string())
            .index(TEST_CLUSTER_ID.to_string())
            .uuid(TEST_FOLDER_UUID.to_string())
            .pri(Some(1))
            .rep(Some(1))
            .docs_count(Some(0))
            .docs_deleted(Some(2))
            .store_size(Some("23812".to_string()))
            .pri_store_size(Some("23812".to_string()))
            .name(Some(TEST_FOLDER_NAME.to_string()))
            .build()
            .unwrap()
    }
}

impl TestExample<Document> for Document {
    fn test_example(_val: Option<&str>) -> Document {
        let created = NaiveDateTime::default()
            .with_year(2024)
            .unwrap()
            .with_month(4)
            .unwrap()
            .with_day(3)
            .unwrap()
            .with_hour(13)
            .unwrap()
            .with_minute(51)
            .unwrap()
            .with_second(32)
            .unwrap()
            .and_utc();

        let modified = NaiveDateTime::default()
            .with_year(2024)
            .unwrap()
            .with_month(4)
            .unwrap()
            .with_day(25)
            .unwrap()
            .with_hour(11)
            .unwrap()
            .with_minute(14)
            .unwrap()
            .with_second(55)
            .unwrap()
            .and_utc();

        DocumentBuilder::default()
            .folder_id(TEST_FOLDER_ID.to_string())
            .folder_path(TEST_FOLDER_PATH.to_string())
            .document_id(DOCUMENT_ID.to_string())
            .document_ssdeep(DOCUMENT_SSDEEP_HASH.to_string())
            .document_name(DOCUMENT_NAME.to_string())
            .document_path(DOCUMENT_PATH.to_string())
            .document_size(35345)
            .document_type(DOCUMENT_TYPE.to_string())
            .document_extension(DOCUMENT_EXTENSION.to_string())
            .document_permissions(777)
            .content(DOCUMENT_CONTENT.to_string())
            .document_created(Some(created))
            .document_modified(Some(modified))
            .quality_recognition(Some(10000))
            .highlight(None)
            .embeddings(Some(vec![]))
            .build()
            .unwrap()
    }
}

impl TestExample<DocumentPreview> for DocumentPreview {
    fn test_example(_val: Option<&str>) -> DocumentPreview {
        let document = Document::test_example(None);
        DocumentPreview::from(&document)
    }
}

impl TestExample<DocumentVectors> for DocumentVectors {
    fn test_example(_value: Option<&str>) -> DocumentVectors {
        let local_now = datetime::get_local_now();
        let datetime_now = DateTime::<Utc>::from(local_now);
        DocumentVectors::builder()
            .folder_id(TEST_FOLDER_ID.to_string())
            .document_id(DOCUMENT_ID.to_string())
            .document_name(DOCUMENT_NAME.to_string())
            .document_modified(Some(datetime_now))
            .embeddings(vec![EmbeddingsVector::default()])
            .match_score(None)
            .build()
            .unwrap()
    }
}

impl<T> TestExample<Paginated<Vec<T>>> for Paginated<Vec<T>>
where
    T: DocumentsTrait + TestExample<T> + serde::Serialize,
{
    fn test_example(_value: Option<&str>) -> Paginated<Vec<T>> {
        Paginated::new_with_id(vec![T::test_example(None)], PAGINATE_HASH_ID.to_string())
    }
}

impl TestExample<ScrollNextForm> for ScrollNextForm {
    fn test_example(_value: Option<&str>) -> ScrollNextForm {
        ScrollNextForm::builder()
            .scroll_id(PAGINATE_HASH_ID.to_string())
            .lifetime(SEARCH_SCROLL_LIFETIME.to_string())
            .build()
            .unwrap()
    }
}

impl TestExample<DeleteScrollsForm> for DeleteScrollsForm {
    fn test_example(_value: Option<&str>) -> DeleteScrollsForm {
        DeleteScrollsForm::builder()
            .sessions(vec![PAGINATE_HASH_ID.to_string()])
            .build()
            .unwrap()
    }
}

impl TestExample<SemanticParams> for SemanticParams {
    fn test_example(query: Option<&str>) -> SemanticParams {
        SemanticParams::builder()
            .query(query.unwrap().to_string())
            .query_tokens(None)
            .folder_ids(TEST_FOLDER_ID.to_string())
            .result_size(5)
            .scroll_lifetime(SEARCH_SCROLL_LIFETIME.to_string())
            .knn_amount(Some(5))
            .knn_candidates(Some(100))
            .is_grouped(Some(false))
            .build()
            .unwrap()
    }
}

impl TestExample<FulltextParams> for FulltextParams {
    fn test_example(_value: Option<&str>) -> FulltextParams {
        FulltextParams::builder()
            .query(SEARCH_QUERY.to_string())
            .folder_ids(TEST_FOLDER_ID.to_string())
            .document_type(Some(DOCUMENT_TYPE.to_string()))
            .document_extension(Some(DOCUMENT_EXTENSION.to_string()))
            .created_date_to(Some(SEARCH_CREATED_TO.to_string()))
            .created_date_from(Some(SEARCH_CREATED_FROM.to_string()))
            .scroll_lifetime(SEARCH_SCROLL_LIFETIME.to_string())
            .document_size_to(Some(0))
            .document_size_from(Some(4096))
            .result_size(25)
            .result_offset(0)
            .build()
            .unwrap()
    }
}

impl TestExample<RetrieveParams> for RetrieveParams {
    fn test_example(_value: Option<&str>) -> RetrieveParams {
        RetrieveParams::builder()
            .query(Some(TEST_FOLDER_NAME.to_string()))
            .document_extension(Some(DOCUMENT_EXTENSION.to_string()))
            .created_date_to(Some(SEARCH_CREATED_TO.to_string()))
            .created_date_from(Some(SEARCH_CREATED_FROM.to_string()))
            .document_size_to(Some(0))
            .document_size_from(Some(4096))
            .result_size(25)
            .result_offset(0)
            .is_show_all(true)
            .build()
            .unwrap()
    }
}
