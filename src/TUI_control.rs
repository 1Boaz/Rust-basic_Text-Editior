use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io::{self, stdout, Stdout};


// Main application loop
pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, content: &str) -> io::Result<()> {
    loop {
        // Draw the UI
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new(content.to_string() + "\nPress 'q' to quit.")
                    .block(Block::default().title("My App").borders(Borders::ALL)),
                area,
            );
        })?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    Ok(())
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
