use std::fs::File;
use std::io::{Read, Result};

pub fn get_file_md5(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = [0; 1024];
    let mut context = md5::Context::new();

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        context.consume(&buffer[..bytes_read]);
    }

    let result = context.compute();
    Ok(format!("{:x}", result))
}

#[cfg(test)]
mod tests {
    use crate::utils::md5::get_file_md5;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_calculate_md5() {
        let test_file_path = "/tmp/tmp";
        let content = "aaaaaaassssssssddddddddddfffff\n";
        let mut file = File::create(test_file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let expected_md5 = "7a0cf342efdef012054e2c6dc6967bf6".to_string();

        match get_file_md5(test_file_path) {
            Ok(actual_md5) => {
                // log
                println!("expected_md5: {}", expected_md5);
                println!("actual_md5: {}", actual_md5);
                assert_eq!(expected_md5, actual_md5);
            }
            Err(e) => {
                // log
                println!("Error: {}", e);
                assert!(false);
            }
        }
        std::fs::remove_file(test_file_path).unwrap();
    }
}
