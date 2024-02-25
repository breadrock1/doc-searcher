#![allow(unused_imports, dead_code)]

use dotenv::dotenv;
use pdfium_render::prelude::Pdfium;
use pdfium_render::prelude::PdfiumError;

use std::io::ErrorKind;
use std::path::Path;

pub fn parse(file_path: &Path) -> Result<String, std::io::Error> {
    Ok(parse_by_pdfium(file_path).unwrap_or(String::default()))
}

fn parse_by_pdfium(file_path: &Path) -> Result<String, PdfiumError> {
    dotenv().ok();
    let pdfim_lib_path: String = dotenv::var("PDFIUM_LIBRARY_PATH").unwrap_or(".".to_string());

    let lib_name_at_path = Pdfium::pdfium_platform_library_name_at_path(pdfim_lib_path.as_str());
    let binds =
        Pdfium::bind_to_library(lib_name_at_path).or_else(|_| Pdfium::bind_to_system_library());

    if binds.is_err() {
        let _ = binds.err().unwrap();
        return Err(PdfiumError::IoError(ErrorKind::InvalidInput.into()));
    }

    let pdfium = Pdfium::new(binds.unwrap());
    let file_path_str = file_path.to_str().unwrap();
    let pdf_read_result = pdfium.load_pdf_from_file(file_path_str, None);
    if pdf_read_result.is_err() {
        let _ = pdf_read_result.err().unwrap();
        return Err(PdfiumError::IoError(ErrorKind::InvalidInput.into()));
    }

    let pdf_text_data = pdf_read_result
        .unwrap()
        .pages()
        .iter()
        .enumerate()
        .map(|(_, page)| page.text().unwrap().all())
        .collect::<Vec<String>>()
        .join("\n");

    Ok(pdf_text_data)
}
