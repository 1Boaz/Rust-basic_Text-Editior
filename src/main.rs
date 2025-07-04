mod file_modification;
mod TUI_control;

use std::env::args;
use std::io;
use crate::TUI_control::{restore_terminal, run, setup_terminal};

fn main() -> io::Result<()> {
    // Reading the file name
    let filename = args().nth(1).expect("No file specified, You must specify a filename as an argument.");
    // Reading contents (and creating a file if it doesn't exist)
    let content: String = match file_modification::read_file_content(&filename) {
        Ok(content) => content,
        Err(_) => file_modification::create_file(&*filename)
    };
    let mut terminal = setup_terminal()?;

    // To start with an empty input box:
    let new_text = run(&mut terminal, "")?;
    println!("You entered: {:?}", new_text);

    // To edit existing text:
    let edited_text = run(&mut terminal, &content)?;
    file_modification::write_file_content(&filename, edited_text.unwrap_or(file_modification::create_file(&*filename)))?;

    restore_terminal()?;
    Ok(())
}
