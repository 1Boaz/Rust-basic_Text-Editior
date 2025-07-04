use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io::{self, stdout, Stdout};

struct AppState {
    input: String,
    cursor_position: usize,
}

impl AppState {
    fn with_text(initial_text: &str) -> Self {
        Self {
            input: initial_text.to_string(),
            cursor_position: initial_text.len(),
        }
    }

    // --- START: NEW multi-line cursor logic ---

    // Calculates the (row, col) of the cursor based on its 1D position and newlines.
    fn get_cursor_location(&self) -> (usize, usize) {
        let text_before_cursor = &self.input[..self.cursor_position];
        let row = text_before_cursor.lines().count().saturating_sub(1);
        let col = text_before_cursor.lines().last().unwrap_or("").len();
        (row, col)
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn move_cursor_up(&mut self) {
        let (row, col) = self.get_cursor_location();
        if row > 0 {
            // Get the length of the previous line
            let prev_line_len = self.input.lines().nth(row - 1).unwrap_or("").len();
            // Find the starting position of the previous line
            let prev_line_start: usize = self.input
                .lines()
                .take(row - 1)
                // Add 1 for each newline character
                .map(|line| line.len() + 1)
                .sum();
            // Clamp column to the length of the previous line
            let new_col = col.min(prev_line_len);
            self.cursor_position = prev_line_start + new_col;
        }
    }

    /// Moves the cursor down by one line if possible.
    ///
    /// This function calculates the current row and column of the cursor and checks if the cursor
    /// can move down by comparing the row number with the total number of lines. If the cursor
    /// can move down, it calculates the starting position of the next line and updates the cursor
    /// position, ensuring the column is clamped to the length of the next line.
    fn move_cursor_down(&mut self) {
        let (row, _) = self.get_cursor_location();
        let line_count = self.input.lines().count();
        if row < line_count - 1 {
            let (row, col) = self.get_cursor_location();
            // Get the length of the next line
            let next_line_len = self.input.lines().nth(row + 1).unwrap_or("").len();
            // Find the starting position of the next line
            let next_line_start: usize = self.input
                .lines()
                .take(row + 1)
                .map(|line| line.len() + 1)
                .sum();
            // Clamp column to the length of the next line
            let new_col = col.min(next_line_len);
            self.cursor_position = next_line_start + new_col;
        }
    }

    // --- END: NEW multi-line cursor logic ---

    fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        if self.cursor_position != 0 {
            let current_index = self.cursor_position;
            let from_left_to_current = current_index - 1;
            let before_char_to_delete = self.input.chars().take(from_left_to_current);
            let after_char_to_delete = self.input.chars().skip(current_index);
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }
}

// Run the Text Editor TUI, given a mutable reference to a
// `Terminal<CrosstermBackend<Stdout>>` and an initial text string.
//
// Returns `Ok(Some(text))` if the user saves and exits (with Ctrl+S),
// `Ok(None)` if the user cancels (with Esc), and `Err(err)` on any
// other IO error.
//
// The `Terminal` is expected to be already set up in raw mode, and
// this function will tear it down when it exits.
pub fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    initial_text: &str,
) -> io::Result<Option<String>> {
    let mut app_state = AppState::with_text(initial_text);

    loop {
        terminal.draw(|frame| {
            // The Paragraph widget naturally handles multiline text (`\n`)
            let input_paragraph = Paragraph::new(app_state.input.as_str())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Text Editor"));

            frame.render_widget(input_paragraph, frame.size());

            // --- MODIFIED: Set cursor based on 2D position ---
            let (cursor_row, cursor_col) = app_state.get_cursor_location();
            frame.set_cursor(
                // The +1s account for the widget's border
                frame.size().x + cursor_col as u16 + 1,
                frame.size().y + cursor_row as u16 + 1,
            );
        })?;

        // Handle input to modify the state
        if event::poll(std::time::Duration::from_millis(250))? {
            // --- MODIFIED: Match on the full KeyEvent to check for modifiers like Ctrl ---
            if let Event::Key(key) = event::read()? {
                match key {
                    // Save and Exit on Ctrl+S
                    KeyEvent {
                        code: KeyCode::Char('s'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        return Ok(Some(app_state.input));
                    }
                    // Cancel on Esc
                    KeyEvent {
                        code: KeyCode::Esc, ..
                    } => {
                        return Ok(None);
                    }
                    // Insert newline on Enter
                    KeyEvent {
                        code: KeyCode::Enter, ..
                    } => {
                        app_state.enter_char('\n');
                    }
                    // Handle other character inputs
                    KeyEvent {
                        code: KeyCode::Char(c), ..
                    } => {
                        app_state.enter_char(c);
                    }
                    // Handle deletions
                    KeyEvent {
                        code: KeyCode::Backspace,
                        ..
                    } => {
                        app_state.delete_char();
                    }
                    // Handle cursor movement
                    KeyEvent {
                        code: KeyCode::Left, ..
                    } => {
                        app_state.move_cursor_left();
                    }
                    KeyEvent {
                        code: KeyCode::Right, ..
                    } => {
                        app_state.move_cursor_right();
                    }
                    KeyEvent {
                        code: KeyCode::Up, ..
                    } => {
                        app_state.move_cursor_up();
                    }
                    KeyEvent {
                        code: KeyCode::Down, ..
                    } => {
                        app_state.move_cursor_down();
                    }
                    _ => {}
                }
            }
        }
    }
}

// Helper functions for terminal setup and restoration
pub fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}