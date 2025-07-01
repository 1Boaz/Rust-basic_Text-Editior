use std::{fs, io};

pub fn read_file_content(file: &str) -> Result<String, io::Error> {
    fs::read_to_string(file)
}

pub fn write_file_content(file: &str, content: String) -> Result<(), io::Error> {
    match fs::write(file, &content) {
        Ok(_) => Ok(()),
        Err(_) =>
            match fs::File::create(file) {
                Ok(_) => fs::write(file, &content),
                Err(e) => Err(e)
            }
    }
}