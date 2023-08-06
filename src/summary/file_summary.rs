use crate::utils::file;
use crate::utils::md5;
use std::io::Error;

pub struct FileSummary {
    pub full_file_path: String,
    pub relative_file_path: String,
    pub file_name: String,
    pub md5: String,
    pub file_size: u64,
    pub file_type: file::FileType,
}

pub fn get_file_summary(full_file_path: &str, parent_dir: &str) -> Result<FileSummary, Error> {
    let file_name = file::get_file_name(full_file_path);
    let relative_file_path = file::get_relative_file_path(full_file_path, parent_dir);
    let md5 = md5::get_file_md5(full_file_path)?;
    let file_size = file::get_file_size(full_file_path);
    let file_type = file::get_file_type(full_file_path);

    Ok(FileSummary {
        full_file_path: full_file_path.to_string(),
        relative_file_path,
        file_name,
        md5,
        file_size,
        file_type,
    })
}
