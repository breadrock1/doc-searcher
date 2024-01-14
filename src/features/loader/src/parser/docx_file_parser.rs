use docx_rust::DocxFile;
use std::io::Error;
use std::path::Path;

pub fn parse(file_path: &Path) -> Result<String, Error> {
    let docx = DocxFile::from_file(file_path).unwrap();
    let docx = docx.parse().unwrap();
    let entity_data = docx.document.body.text();
    Ok(entity_data)
}

#[test]
pub fn parse_test() {
    let file_path = Path::new("/Users/breadrock/Downloads/Job-offer.docx");
    let data = parse(file_path).unwrap();
    println!("{}", data);
}
