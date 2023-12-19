// use lopdf::Document;
use std::fs::File;
use std::path::Path;
use std::io::{Error, Read};
use tesseract::*;
use tesseract::plumbing::leptonica_sys::setenv;

pub fn parse(file_path: &Path) -> Result<String, Error> {
    // let document_result = Document::load(file_path);
    // match document_result {
    //     Ok(document) => Ok(parse_by_lopdf(&document)),
    //     Err(err) => {
    //         println!("{err}");
    //         Ok(String::new())
    //     }
    // }

    let file_res = File::open(file_path);
    if file_res.is_err() {
        return Err(file_res.err().unwrap());
    }

    let mut file = file_res.unwrap();
    let mut file_data = String::new();
    file.read_to_string(&mut file_data).unwrap_or_default();

    let engine_mode = OcrEngineMode::Default;
    let mut tess = Tesseract::new_with_data(file_data.as_bytes(), Some("eng"), engine_mode).unwrap();
    tess.set_page_seg_mode(PageSegMode::PsmAuto);
    let mut tess_data = tess.recognize().unwrap();
    let text_data = tess_data.get_text().unwrap();
    Ok(text_data)
}

// fn parse_by_lopdf(document: &Document) -> String {
//     let pages = document.get_pages();
//     let collected_text = pages
//         .iter()
//         .enumerate()
//         .map(|(i, _)| parse_pdf_page(&document, i))
//         .collect::<Vec<String>>();
//
//     collected_text.join("\n")
// }
//
// fn parse_pdf_page(document: &Document, i: usize) -> String {
//     let page_number = (i + 1) as u32;
//     let text = document.extract_text(&[page_number]);
//     text.unwrap_or_default()
// }

#[test]
fn parse_test() {
    let file_path = Path::new("/Users/breadrock/Downloads/test.png");
    let data = parse(&file_path).unwrap();
    println!("{}", data);
}
