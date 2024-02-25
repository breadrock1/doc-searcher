#![allow(unused_imports, dead_code)]

use docx::document::{BodyContent, ParagraphContent, TableCellContent};
use docx::DocxFile;
use std::path::Path;

pub fn parse(file_path: &Path) -> Result<String, std::io::Error> {
    let docx_result = DocxFile::from_file(file_path);
    if docx_result.is_err() {
        let docx_err = docx_result.err().unwrap();
        println!("{:?}", docx_err);
        return Ok(String::default());
    }

    let docx = docx_result.unwrap();
    let docx = docx.parse().unwrap_or_default();
    let collected_text_data = docx
        .document
        .body
        .content
        .iter()
        .map(|bocy_content| match bocy_content {
            BodyContent::Paragraph(par) => extract_paragraph_text(par),
            BodyContent::Table(tab) => extract_table_text(tab),
        })
        .collect::<Vec<String>>();

    Ok(collected_text_data.join(""))
}

fn extract_table_text(table: &docx::document::Table) -> String {
    let table_data = table
        .rows
        .iter()
        .flat_map(|row| {
            row.cells
                .iter()
                .map(extract_table_cell_data)
                .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>();

    table_data.join("")
}

fn extract_table_cell_data(table_cell: &docx::document::TableCell) -> String {
    let table_cell_data = table_cell
        .content
        .iter()
        .flat_map(|tcc| match tcc {
            TableCellContent::Paragraph(par) => extract_cell_content(par),
        })
        .collect::<Vec<String>>();

    table_cell_data.join("")
}

fn extract_cell_content(paragraph: &docx::document::Paragraph) -> Vec<String> {
    paragraph.iter_text().map(|pat| pat.to_string()).collect()
}

fn extract_paragraph_text(paragraph: &docx::document::Paragraph) -> String {
    let paragraph_data = paragraph
        .content
        .iter()
        .map(|par| match par {
            ParagraphContent::Run(run) => run.iter_text().map(|cont| cont.to_string()).collect(),
            ParagraphContent::Link(link) => {
                link.content.iter_text().map(|l| l.to_string()).collect()
            }
            _ => String::default(),
        })
        .collect::<Vec<String>>();

    paragraph_data.join("")
}
