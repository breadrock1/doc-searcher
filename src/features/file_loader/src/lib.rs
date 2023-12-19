mod file_data;

pub use crate::file_data::FileData;
use crate::file_data::FileDataBuilder;

use chrono::{DateTime, Utc};
use hasher::{gen_hash, HashType};

use std::ffi::OsStr;
use std::fs::File;
use std::io::{Error, Read};
use std::os::unix::fs::MetadataExt;
use std::os::unix::prelude::PermissionsExt;
use std::path::Path;
use std::time::SystemTime;

pub fn load_directory_entity(directory: &Path) -> Vec<FileData> {
    if directory.is_file() {
        let loaded_result = load_target_file(&directory);
        return match loaded_result {
            Ok(document) => vec![document],
            Err(_) => Vec::default(),
        };
    }

    walkdir::WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .flat_map(|e| load_directory_entity(e.path()))
        .collect()
}

fn load_target_file(file_path: &Path) -> Result<FileData, Error> {
    let file_res = File::open(file_path);
    if file_res.is_err() {
        return Err(file_res.err().unwrap());
    }

    let mut file = file_res.unwrap();
    let metadata_res = file.metadata();
    if metadata_res.is_err() {
        return Err(metadata_res.err().unwrap());
    }

    let file_metadata = metadata_res.unwrap();
    let perms_ = file_metadata.permissions().mode();

    let file_path_ = file_path.to_str().unwrap_or("unknown");

    let file_name_ = file_path
        .file_name()
        .unwrap_or(OsStr::new(file_path_))
        .to_str()
        .unwrap_or("unknown");

    let ext_ = file_path
        .extension()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap_or("unknown");

    let mut file_data_ = String::new();
    file.read_to_string(&mut file_data_).unwrap_or_default();
    let md5_hash = gen_hash(HashType::MD5, file_data_.as_bytes());
    let binding = md5_hash.unwrap();
    let md5_hash_ = binding.get_hash_data();

    let ssdeep_hash = gen_hash(HashType::SSDEEP, file_data_.as_bytes());
    let binding = ssdeep_hash.unwrap();
    let ssdeep_hash_ = binding.get_hash_data();

    // POSIX does not require a Unix system to support including file creation timestamps, only timestamps for
    // last access (atime), last modification (mtime), and last status change (ctime). Some Unix systems do
    // include this information using a field often referred to as "btime" (birth time).
    //
    // However, Linux does not expose this value in struct stat, and it is not supported on all file systems.
    // Notably, ext4, xfs, and btrfs do, which are some of the most common file systems, but Linux supports a
    // variety of file systems which do not expose this information currently (and which may or may not support
    // it at all), including various varieties of FAT, NTFS, and UDF.
    //
    // It is possible to use the statx system call, but apparently Rust doesn't support that at this moment,
    // and it was only added in 2016, which means there are still OSes which probably wouldn't support it
    // (RHEL 7, for example, came out in 2014). Both glibc and musl have support for it, but musl, which is used
    // by Alpine, only added it in 2020, which may be too new to be depended on by the Rust standard library.
    // Typically, the most portable way to get the latest file or directory is the modification time. That's
    // available nearly universally, including in POSIX. The access time is probably less useful and is often
    // disabled for performance reasons.
    let dt_cr_utc: DateTime<Utc> = file_metadata
        .created()
        .unwrap_or_else(get_local_datetime)
        .into();

    let dt_md_utc: DateTime<Utc> = file_metadata
        .modified()
        .unwrap_or_else(get_local_datetime)
        .into();

    let built_file_data = FileDataBuilder::default()
        .bucket_uuid("common_bucket".to_string())
        .bucket_path("/".to_string())
        .document_name(file_name_.to_string())
        .document_path(file_path_.to_string())
        .document_size(file_metadata.size() as i32)
        .document_type("document".to_string())
        .document_extension(ext_.to_string())
        .document_permissions(perms_ as i32)
        .document_md5_hash(md5_hash_.to_string())
        .document_ssdeep_hash(ssdeep_hash_.to_string())
        .entity_data(file_data_)
        .entity_keywords(Vec::<String>::default())
        .document_created(Some(dt_cr_utc))
        .document_modified(Some(dt_md_utc))
        .build();

    Ok(built_file_data.unwrap())
}

fn get_local_datetime(err: Error) -> SystemTime {
    println!("{:?}", err);
    SystemTime::now()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
