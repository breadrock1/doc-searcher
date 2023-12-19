use std::fs::File;
use std::path::Path;
use std::io::{Error, Read};

pub fn parse(file_path: &Path) -> Result<String, Error> {
    let file_res = File::open(file_path);
    if file_res.is_err() {
        return Err(file_res.err().unwrap());
    }

    let mut file = file_res.unwrap();
    let mut file_data = String::new();
    file.read_to_string(&mut file_data).unwrap_or_default();
    Ok(file_data)
}
