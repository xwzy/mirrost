use std::fs::File;

#[derive(Debug, PartialEq)]
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
    // if path is dir, return 0
    if get_file_type(full_file_path) != FileType::File {
        return 0;
    }

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

#[cfg(test)]
mod test {
    use std::fs;
    use std::io::Write;

    use crate::utils::file::{
        get_file_name, get_file_size, get_file_type, get_relative_file_path, FileType,
    };

    #[test]
    fn test_get_file_name() {
        let temp_file_path = "/tmp/mirrost_tmp";
        let test_content = "This is a test content.";
        let mut temp_file = fs::File::create(temp_file_path).unwrap();
        temp_file.write_all(test_content.as_bytes()).unwrap();
        let result = get_file_name(temp_file_path);
        assert_eq!(result, "mirrost_tmp");
        fs::remove_file(temp_file_path).unwrap();
    }

    #[test]
    fn test_get_relative_file_path() {
        let temp_file_path = "/tmp/mirrost_tmp"; // 临时文件路径，根据你的需要修改
        let parent_dir = "/tmp"; // 父目录路径，根据你的需要修改
        let test_content = "This is a test content.";
        let mut temp_file = fs::File::create(temp_file_path).unwrap();
        temp_file.write_all(test_content.as_bytes()).unwrap();
        let result = get_relative_file_path(temp_file_path, parent_dir);
        assert_eq!(result, "mirrost_tmp");
        fs::remove_file(temp_file_path).unwrap();
    }

    #[test]
    fn test_get_relative_file_path_invalid_path() {
        let result = get_relative_file_path("/path/that/does/not/exist", "/tmp");
        assert_eq!(result, "");
    }

    #[test]
    fn test_get_file_size() {
        let temp_file_path = "/tmp/mirrost_tmp";
        let test_content = "This is a test content.";
        let mut temp_file = fs::File::create(temp_file_path).unwrap();
        temp_file.write_all(test_content.as_bytes()).unwrap();
        let result = get_file_size(temp_file_path);
        assert_eq!(result, test_content.len() as u64);
        fs::remove_file(temp_file_path).unwrap();
    }

    #[test]
    fn test_get_file_size_nonexistent_file() {
        let result = get_file_size("/path/that/does/not/exist");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_file_type_dir() {
        let temp_dir_path = "/tmp/mirrost_tmp_dir";
        fs::create_dir(temp_dir_path).unwrap();
        let result = get_file_type(temp_dir_path);
        assert_eq!(result, FileType::Dir);
        fs::remove_dir(temp_dir_path).unwrap();
    }

    #[test]
    fn test_get_file_type_file() {
        let temp_file_path = "/tmp/mirrost_tmp_file";
        let test_content = "This is a test content.";
        let mut temp_file = fs::File::create(temp_file_path).unwrap();
        temp_file.write_all(test_content.as_bytes()).unwrap();
        let result = get_file_type(temp_file_path);
        assert_eq!(result, FileType::File);
        fs::remove_file(temp_file_path).unwrap();
    }

    #[test]
    fn test_get_file_type_nonexistent_path() {
        let result = get_file_type("/path/that/does/not/exist");
        assert_eq!(result, FileType::Other);
    }
}
