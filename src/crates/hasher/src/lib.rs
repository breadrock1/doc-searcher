mod errors;
mod hasher;

use crate::errors::{HasherError, HasherResult};
use crate::hasher::Hashed;

use std::fs::File;
use std::io::Read;
use std::string::ToString;

pub enum HashType {
    MD5,
    SSDEEP,
}

pub fn gen_hash(hash_type: HashType, data: &[u8]) -> HasherResult {
    match hash_type {
        HashType::MD5 => md5_hash(data),
        HashType::SSDEEP => ssdeep_hash(data),
    }
}

pub fn gen_hash_from_file(hash_type: HashType, file_path: &str) -> HasherResult {
    let opened_file_result = File::open(file_path);
    if opened_file_result.is_err() {
        return Err(HasherError::FileNotExist(file_path.to_string()));
    }

    let mut opened_file = opened_file_result.unwrap();
    let mut buffer = Vec::new();
    let read_result = opened_file.read_to_end(&mut buffer);
    if read_result.is_err() {
        return Err(HasherError::ReadFileErr(file_path.to_string()));
    }

    gen_hash(hash_type, buffer.as_slice())
}

pub fn compare_ssdeep_hashes(hash_1: &str, hash_2: &str) -> bool {
    let result = ssdeep::compare(hash_1.as_bytes(), hash_2.as_bytes()).unwrap_or(0);
    result != 0
}

#[inline]
pub fn gen_uuid() -> String {
    uuid7::uuid4()
        .encode_hex()
        .to_string()
}

fn ssdeep_hash(data: &[u8]) -> HasherResult {
    match ssdeep::hash(data) {
        None => Err(HasherError::FailedErr),
        Some(hashed) => Ok(Hashed::new(hashed)),
    }
}

fn md5_hash(data: &[u8]) -> HasherResult {
    let digest = md5::compute(data);
    let hash_data = format!("{:x}", digest);
    Ok(Hashed::new(hash_data))
}
