use crate::utils::file;
use crate::utils::md5;
use std::io::Error;

#[derive(Debug, PartialEq)]
pub struct NodeSummary {
    pub full_file_path: String,
    pub relative_file_path: String,
    pub file_name: String,
    pub md5: String,
    pub file_size: u64,
    pub file_type: file::FileType,
}

pub fn get_node_summary(full_file_path: &str, parent_dir: &str) -> Result<NodeSummary, Error> {
    let file_name = file::get_file_name(full_file_path);
    let relative_file_path = file::get_relative_file_path(full_file_path, parent_dir);
    let md5 = md5::get_file_md5(full_file_path)?;
    let file_size = file::get_file_size(full_file_path);
    let file_type = file::get_file_type(full_file_path);

    Ok(NodeSummary {
        full_file_path: full_file_path.to_string(),
        relative_file_path,
        file_name,
        md5,
        file_size,
        file_type,
    })
}

#[cfg(test)]
mod test {
    use crate::summary::node_summary::{get_node_summary, NodeSummary};
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_get_file_summary() {
        let temp_file_path = "/tmp/mirrost_tmp";
        let test_content = "This is a test content.";
        // remove if exist
        std::fs::remove_file(temp_file_path).unwrap_or_default();
        let mut temp_file = File::create(temp_file_path).unwrap();
        temp_file.write_all(test_content.as_bytes()).unwrap();

        let parent_dir = "/tmp";
        let expected_file_summary = NodeSummary {
            full_file_path: temp_file_path.to_string(),
            relative_file_path: "mirrost_tmp".to_string(),
            file_name: "mirrost_tmp".to_string(),
            md5: "25a464d7bde51f9a4325085a5c0d2a71".to_string(),
            file_size: 23,
            file_type: crate::utils::file::FileType::File,
        };

        match get_node_summary(temp_file_path, parent_dir) {
            Ok(actual_file_summary) => {
                println!("expected_file_summary: {:?}", expected_file_summary);
                println!("actual_file_summary: {:?}", actual_file_summary);
                assert_eq!(expected_file_summary, actual_file_summary);
            }
            Err(e) => {
                println!("Error: {}", e);
                assert!(false);
            }
        }
        std::fs::remove_file(temp_file_path).unwrap();
    }

    #[test]
    fn test_get_file_summary_dir() {
        let temp_dir_path = "/tmp/mirrost_tmp_dir";
        // remove if exist
        std::fs::remove_dir(temp_dir_path).unwrap_or_default();
        std::fs::create_dir(temp_dir_path).unwrap();

        let parent_dir = "/tmp";
        let expected_file_summary = NodeSummary {
            full_file_path: temp_dir_path.to_string(),
            relative_file_path: "mirrost_tmp_dir".to_string(),
            file_name: "mirrost_tmp_dir".to_string(),
            md5: "".to_string(),
            file_size: 0,
            file_type: crate::utils::file::FileType::Dir,
        };

        match get_node_summary(temp_dir_path, parent_dir) {
            Ok(actual_file_summary) => {
                println!("expected_file_summary: {:?}", expected_file_summary);
                println!("actual_file_summary: {:?}", actual_file_summary);
                assert_eq!(expected_file_summary, actual_file_summary);
            }
            Err(e) => {
                println!("Error: {}", e);
                assert!(false);
            }
        }
        std::fs::remove_dir(temp_dir_path).unwrap();
    }
}
