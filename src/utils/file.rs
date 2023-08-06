pub enum FileType {
    Dir,
    File,
    Other,
}

pub fn get_file_name(full_file_path: &str) -> String {
    let path = std::path::Path::new(full_file_path);
    match path.file_name() {
        Some(file_name) => file_name.to_str().unwrap().to_string(),
        None => "".to_string(),
    }
}

pub fn get_relative_file_path(full_file_path: &str, parent_dir: &str) -> String {
    let path = std::path::Path::new(full_file_path);
    match path.strip_prefix(parent_dir) {
        Ok(relative_path) => relative_path.to_str().unwrap().to_string(),
        Err(_) => "".to_string(),
    }
}

pub fn get_file_size(full_file_path: &str) -> u64 {
    let path = std::path::Path::new(full_file_path);
    match path.metadata() {
        Ok(metadata) => metadata.len(),
        Err(_) => 0,
    }
}

pub fn get_file_type(full_file_path: &str) -> FileType {
    let path = std::path::Path::new(full_file_path);
    match path.metadata() {
        Ok(metadata) => {
            if metadata.is_dir() {
                FileType::Dir
            } else if metadata.is_file() {
                FileType::File
            } else {
                FileType::Other
            }
        }
        Err(_) => FileType::Other,
    }
}
