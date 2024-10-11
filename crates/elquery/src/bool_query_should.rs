use serde_json::Value;

pub struct BoolQueryShould {
    should: Vec<Value>,
}

pub trait BoolQueryShouldItemTrait {
    fn create(value: Value) -> Value;
}

impl BoolQueryShould {
    pub fn with_multi_match<T>(mut self, filter: Value) -> Self
    where
        T: BoolQueryShouldItemTrait + serde::Serialize,
    {
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
