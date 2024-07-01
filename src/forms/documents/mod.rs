pub(crate) mod document;
pub(crate) mod forms;
pub(crate) mod metadata;
pub(crate) mod preview;
pub(crate) mod schema;
pub(crate) mod vector;

pub(crate) trait DocumentsTrait {
    fn get_folder_id(&self) -> &str;
    fn get_doc_id(&self) -> &str;
    fn set_folder_id(&mut self, folder_id: &str);
}
