mod file_modification;

use std::env::args;
use iocraft::prelude::*;

fn main() {
    // Reading the file name
    let filename = args().nth(1).expect("No file specified, You must specify a filename as an argument.");
    // Reading contents (and creating a file if it doesn't exist)
    let content: String = match file_modification::read_file_content(&filename) {
        Ok(content) => content,
        Err(_) => file_modification::create_file(&*filename)
    };
    // Display file content on screen using iocraft
    element! {
        View(
            border_style: BorderStyle::Round,
            border_color: Color::Blue,
        ) {
            Text(content: content)
        }
    }
        .print();
    
}
