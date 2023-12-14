use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Builder)]
pub struct Bucket {
    pub health: String,
    pub status: String,
    pub index: String,
    pub uuid: String,
    #[serde(alias = "docs.count")]
    pub docs_count: String,
    #[serde(alias = "docs.deleted")]
    pub docs_deleted: String,
    #[serde(alias = "store.size")]
    pub store_size: String,
    #[serde(alias = "pri.store.size")]
    pub pri_store_size: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct BucketForm {
    bucket_name: String,
}

impl ToString for BucketForm {
    fn to_string(&self) -> String {
        let self_data = &self.bucket_name;
        self_data.clone()
    }
}

impl BucketForm {
    pub fn get_name(&self) -> &str {
        self.bucket_name.as_str()
    }
}
