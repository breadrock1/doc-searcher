use crate::forms::TestExample;
use crate::forms::documents::DocumentsTrait;

use derive_builder::Builder;
use serde_derive::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Builder, ToSchema)]
pub struct Paginated<D> {
    #[schema(value_type = Paginated<Vec<Document>>)]
    founded: D,
    #[schema(example = "10m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scroll_id: Option<String>,
}

impl<D> Paginated<D> {
    pub fn new(founded: D) -> Self {
        Paginated {
            founded,
            scroll_id: None,
        }
    }
    pub fn new_with_id(founded: D, id: String) -> Self {
        Paginated {
            founded,
            scroll_id: Some(id),
        }
    }
    pub fn new_with_opt_id(founded: D, scroll_id: Option<String>) -> Self {
        Paginated { founded, scroll_id }
    }
    pub fn get_founded(&self) -> &D {
        &self.founded
    }
    pub fn get_founded_mut(&mut self) -> &mut D {
        &mut self.founded
    }
    pub fn get_scroll_id(&self) -> Option<&String> {
        self.scroll_id.as_ref()
    }
}

impl <T> TestExample<Paginated<Vec<T>>> for Paginated<Vec<T>>
where 
    T: DocumentsTrait + TestExample<T>,
{
    fn test_example(_value: Option<&str>) -> Paginated<Vec<T>> {
        let id = "DXF1ZXJ5QW5kRmV0Y2gBAD4WYm9laVYtZndUQlNsdDcwakFMNjU1QQ==";
        Paginated::new_with_id(
            vec![T::test_example(None)],
            id.to_string()
        )
    }
}
