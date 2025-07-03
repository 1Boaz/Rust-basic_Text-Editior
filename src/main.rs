mod file_modification;
mod TUI_control;

use std::env::args;
use std::io;


fn main() -> io::Result<()> {
    // Reading the file name
    let filename = args().nth(1).expect("No file specified, You must specify a filename as an argument.");
    // Reading contents (and creating a file if it doesn't exist)
    let content: String = match file_modification::read_file_content(&filename) {
        Ok(content) => content,
        Err(_) => file_modification::create_file(&*filename)
    };
        // 1. Setup the terminal
    let mut terminal = TUI_control::setup_terminal()?;

    // 2. Run the application loop
    TUI_control::run(&mut terminal, content.as_str())?;

    // 3. Restore the terminal
    TUI_control::restore_terminal()?;

    Ok(())
}