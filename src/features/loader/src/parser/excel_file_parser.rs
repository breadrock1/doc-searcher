use office::{DataType, Excel, Range};

use std::io::Error;
use std::path::Path;

pub fn parse(file_path: &Path) -> Result<String, Error> {
    let mut workbook = Excel::open(file_path).unwrap();
    let sheet_data = workbook
        .sheet_names()
        .unwrap_or_default()
        .iter()
        .flat_map(|sheet_name| {
            let range_result = workbook.worksheet_range(&sheet_name);
            let range = range_result.unwrap_or_default();
            parse_sheet(&range)
        })
        .collect::<Vec<String>>();

    let all_sheets_data = sheet_data.join("\n");
    Ok(all_sheets_data)
}

fn parse_sheet(range: &Range) -> Vec<String> {
    range
        .rows()
        .enumerate()
        .flat_map(|(row_index, row)| parse_sheet_row(range, row_index, row))
        .collect::<Vec<String>>()
}

fn parse_sheet_row(range: &Range, row_index: usize, row_slice: &[DataType]) -> Vec<String> {
    row_slice
        .iter()
        .enumerate()
        .filter(|(_, cell)| cell != &&DataType::Empty)
        .map(|(cell_index, _)| range.get_value(row_index, cell_index))
        .map(|data_type| extract_data_type_value(data_type))
        .collect::<Vec<String>>()
}

fn extract_data_type_value(data_type: &DataType) -> String {
    match data_type {
        DataType::Int(value) => value.to_string(),
        DataType::Bool(value) => value.to_string(),
        DataType::Float(value) => value.to_string(),
        DataType::String(value) => value.to_string(),
        DataType::Empty | DataType::Error(_) => String::new(),
    }
}
