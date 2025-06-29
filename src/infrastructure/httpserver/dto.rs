use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct PaginateQuery {
    lifetime: Option<String>
}

impl PaginateQuery {
    pub fn lifetime(&self) -> String {
        self.lifetime.clone().unwrap_or("5m".to_string())
    }
}
