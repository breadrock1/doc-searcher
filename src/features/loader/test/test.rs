extern crate loader;

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn load_directory_entity_test() {
        let file_path = "./src/features/loader/resources";
        let path_object = Path::new(file_path);
        let documents = loader::load_directory_entity(&path_object)
            .into_iter()
            .map(|file_data| file_data.document_name.clone())
            .collect::<Vec<String>>();

        assert_eq!("file_1.txt", documents.first().unwrap().as_str());
    }

    #[test]
    fn load_file_entity_test() {
        let file_path = "./src/features/loader/resources/file_1.txt";
        let path_object = Path::new(file_path);
        let documents = loader::load_directory_entity(&path_object)
            .into_iter()
            .map(|file_data| file_data.document_name.clone())
            .collect::<Vec<String>>();

        assert_eq!("file_1.txt", documents.first().unwrap().as_str());
    }
}
