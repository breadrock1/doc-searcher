extern crate loader;

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn load_directory_entity_test() {
        let file_path = "./src/crates/loader/resources";
        let path_object = Path::new(file_path);
        let documents = loader::load_passed_file_by_path(&path_object)
            .into_iter()
            .filter(|file_data| file_data.document_extension.as_str().eq("txt"))
            .map(|file_data| file_data.document_name.clone())
            .collect::<Vec<String>>();

        assert_eq!("test.txt", documents.first().unwrap().as_str());
    }

    #[test]
    fn test_load_txt_file_entity() {
        let file_path = "./src/crates/loader/resources/test.txt";
        let path_object = Path::new(file_path);
        let documents = loader::load_passed_file_by_path(&path_object)
            .into_iter()
            .map(|file_data| file_data.content.clone())
            .collect::<Vec<String>>();

        let extracted_data_size = documents.first().unwrap().len();
        assert!(extracted_data_size < 1050);
    }

    #[test]
    fn test_load_docx_file_entity() {
        let file_path = "./src/crates/loader/resources/test.docx";
        let path_object = Path::new(file_path);
        let documents = loader::load_passed_file_by_path(&path_object)
            .into_iter()
            .map(|file_data| file_data.content.clone())
            .collect::<Vec<String>>();

        let extracted_data_size = documents.first().unwrap().len();
        assert!(extracted_data_size < 1050);
    }

    #[test]
    fn test_load_excel_file_entity() {
        let file_path = "./src/crates/loader/resources/test.xls";
        let path_object = Path::new(file_path);
        let documents = loader::load_passed_file_by_path(&path_object)
            .into_iter()
            .map(|file_data| file_data.content.clone())
            .collect::<Vec<String>>();

        let extracted_data_size = documents.first().unwrap().len();
        assert!(extracted_data_size < 1050);
    }

    #[test]
    fn test_load_pdf_file_entity() {
        let file_path = "./src/crates/loader/resources/test.pdf";
        let path_object = Path::new(file_path);
        let documents = loader::load_passed_file_by_path(&path_object)
            .into_iter()
            .map(|file_data| file_data.content.clone())
            .collect::<Vec<String>>();

        let extracted_data_size = documents.first().unwrap().len();
        assert!(extracted_data_size < 1050);
    }
}
