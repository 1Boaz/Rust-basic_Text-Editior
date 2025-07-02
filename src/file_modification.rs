use std::{fs, io};

pub fn read_file_content(file: &str) -> Result<String, io::Error> {
    fs::read_to_string(file)
}

pub fn write_file_content(file: &str, content: String) -> Result<(), io::Error> {
    fs::write(file, &content)
}

pub fn create_file(file: &str) -> String {
    fs::File::create(file).expect("Unable to create file, Please check your permissions and try again.");
    "".to_string()
}