use crate::orchestra::forms::CreateClusterForm;
use crate::orchestra::models::Cluster;

use crate::searcher::forms::{AllRecordsParams, DeletePaginationsForm, FulltextParams, PaginateNextForm, SemanticParams};
use crate::searcher::models::{Paginated, SearchParams};
use crate::storage::models::{Document, DocumentBuilder, DocumentPreview, DocumentVectors};
use crate::storage::models::{Artifacts, EmbeddingsVector, GroupValue, OcrMetadata};
use crate::storage::DocumentsTrait;
use crate::storage::forms::{CreateFolderForm, FolderType};
use crate::storage::models::Folder;

use chrono::{Datelike, DateTime, NaiveDateTime, Timelike, Utc};

const TEST_CLUSTER_NAME: &str = "test-slave-cluster";
const TEST_CLUSTER_ROLE: &str = "slave";
const TEST_CLUSTER_IP: &str = "172.19.0.2";
const TEST_CLUSTER_ID: &str = "d93df49fa6ff";

const TEST_FOLDER_ID: &str = "test-folder";
const TEST_FOLDER_UUID: &str = "fDdHOrwMSESM9OlhLsrMWQ";
const TEST_FOLDER_NAME: &str = "Test Folder";
const TEST_FOLDER_PATH: &str = "./indexer/test-folder";

const DOCUMENT_OCR_JOB_ID: &str = "c643c506-f5c3-4262-991d-bbe847035499";
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

impl TestExample<CreateClusterForm> for CreateClusterForm {
    fn test_example(_value: Option<&str>) -> CreateClusterForm {
        CreateClusterForm::builder()
            .cluster_id(TEST_CLUSTER_NAME.to_string())
            .role(TEST_CLUSTER_ROLE.to_string())
            .build()
            .unwrap()
    }
}

impl TestExample<Cluster> for Cluster {
    fn test_example(_value: Option<&str>) -> Cluster {
        Cluster::builder()
            .ip(TEST_CLUSTER_IP.to_string())
            .heap_percent("32".to_string())
            .ram_percent("67".to_string())
            .cpu("2".to_string())
            .node_role("cdfhilmrstw".to_string())
            .master("*".to_string())
            .name(TEST_CLUSTER_ID.to_string())
            .build()
            .unwrap()
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
            .pri(Some("1".to_string()))
            .rep(Some("1".to_string()))
            .docs_count(Some("0".to_string()))
            .docs_deleted(Some("2".to_string()))
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

        let group_values = vec![GroupValue::builder()
            .name("Date of TN".to_string())
            .json_name("date_of_tn".to_string())
            .group_type("string".to_string())
            .value(Some("2023-10-29".to_string()))
            .build()
            .unwrap()];

        let artifacts = vec![Artifacts::builder()
            .group_name("Information of TN".to_string())
            .group_json_name("tn_info".to_string())
            .group_values(Some(group_values))
            .build()
            .unwrap()];

        let ocr_metadata = OcrMetadata::builder()
            .job_id(DOCUMENT_OCR_JOB_ID.to_string())
            .pages_count(1)
            .doc_type("TN".to_string())
            .artifacts(Some(artifacts))
            .build()
            .unwrap();

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
            .ocr_metadata(Some(ocr_metadata))
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
    T: DocumentsTrait + TestExample<T>,
{
    fn test_example(_value: Option<&str>) -> Paginated<Vec<T>> {
        Paginated::new_with_id(vec![T::test_example(None)], PAGINATE_HASH_ID.to_string())
    }
}


impl TestExample<PaginateNextForm> for PaginateNextForm {
    fn test_example(_value: Option<&str>) -> PaginateNextForm {
        PaginateNextForm::builder()
            .scroll_id(PAGINATE_HASH_ID.to_string())
            .lifetime(SEARCH_SCROLL_LIFETIME.to_string())
            .build()
            .unwrap()
    }
}

impl TestExample<DeletePaginationsForm> for DeletePaginationsForm {
    fn test_example(_value: Option<&str>) -> DeletePaginationsForm {
        DeletePaginationsForm::builder()
            .sessions(vec![PAGINATE_HASH_ID.to_string()])
            .build()
            .unwrap()
    }
}

impl TestExample<SearchParams> for SearchParams {
    fn test_example(query: Option<&str>) -> SearchParams {
        SearchParams::builder()
            .query(query.unwrap().to_string())
            .folder_ids(Some(TEST_FOLDER_ID.to_string()))
            .document_type(DOCUMENT_TYPE.to_string())
            .document_extension(DOCUMENT_EXTENSION.to_string())
            .created_date_to(SEARCH_CREATED_TO.to_string())
            .created_date_from(SEARCH_CREATED_FROM.to_string())
            .document_size_to(37000)
            .document_size_from(0)
            .result_size(25)
            .result_offset(0)
            .scroll_lifetime(SEARCH_SCROLL_LIFETIME.to_string())
            .knn_amount(Some(5))
            .knn_candidates(Some(100))
            .show_all(Some(true))
            .build()
            .unwrap()
    }
}

impl TestExample<FulltextParams> for FulltextParams {
    fn test_example(_value: Option<&str>) -> FulltextParams {
        FulltextParams::builder()
            .query(SEARCH_QUERY.to_string())
            .folder_ids(Some(TEST_FOLDER_ID.to_string()))
            .document_type(DOCUMENT_TYPE.to_string())
            .document_extension(DOCUMENT_EXTENSION.to_string())
            .created_date_to(SEARCH_CREATED_TO.to_string())
            .created_date_from(SEARCH_CREATED_FROM.to_string())
            .scroll_lifetime(SEARCH_SCROLL_LIFETIME.to_string())
            .document_size_to(0)
            .document_size_from(4096)
            .result_size(25)
            .result_offset(0)
            .build()
            .unwrap()
    }
}

impl TestExample<AllRecordsParams> for AllRecordsParams {
    fn test_example(_value: Option<&str>) -> AllRecordsParams {
        AllRecordsParams::builder()
            .query(TEST_FOLDER_NAME.to_string())
            .folder_id(Some(TEST_FOLDER_ID.to_string()))
            .document_type(DOCUMENT_TYPE.to_string())
            .document_extension(DOCUMENT_EXTENSION.to_string())
            .created_date_to(SEARCH_CREATED_TO.to_string())
            .created_date_from(SEARCH_CREATED_FROM.to_string())
            .scroll_lifetime(SEARCH_SCROLL_LIFETIME.to_string())
            .document_size_to(0)
            .document_size_from(4096)
            .result_size(25)
            .build()
            .unwrap()
    }
}

impl TestExample<SemanticParams> for SemanticParams {
    fn test_example(_value: Option<&str>) -> SemanticParams {
        SemanticParams::builder()
            .query(SEARCH_QUERY.to_string())
            .folder_ids(Some(TEST_FOLDER_ID.to_string()))
            .document_size_from(0)
            .knn_amount(Some(5))
            .knn_candidates(Some(100))
            .scroll_lifetime(SEARCH_SCROLL_LIFETIME.to_string())
            .build()
            .unwrap()

    }
}
