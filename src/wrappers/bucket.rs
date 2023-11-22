use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
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

#[allow(clippy::too_many_arguments)]
impl Bucket {
    pub fn new(
        health: String,
        status: String,
        index: String,
        uuid: String,
        docs_count: String,
        docs_deleted: String,
        store_size: String,
        pri_store_size: String,
        pri: Option<String>,
        rep: Option<String>,
    ) -> Self {
        Bucket {
            health,
            status,
            index,
            uuid,
            docs_count,
            docs_deleted,
            store_size,
            pri_store_size,
            pri,
            rep,
        }
    }
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
