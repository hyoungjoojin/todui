use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, CompletedFrame, Frame, Terminal};
use std::io::{self, Result};

pub struct Canvas {
    terminal: Terminal<CrosstermBackend<io::Stderr>>,
}

impl Canvas {
    pub fn new() -> Result<Canvas> {
        match enable_raw_mode() {
            Ok(_) => {}
            Err(error) => return Err(error),
        }

        execute!(io::stderr(), EnterAlternateScreen).expect("failed to enter alternate screen");
        execute!(io::stderr(), EnableMouseCapture).expect("failed to enable mouse capture");

        let backend = CrosstermBackend::new(io::stderr());

        Ok(Canvas {
            terminal: Terminal::new(backend).expect("failed to initialize terminal"),
        })
    }

    pub fn draw<F>(&mut self, f: F) -> Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame),
    {
        self.terminal.draw(f)
    }

    pub fn clear(&mut self) {
        disable_raw_mode().expect("failed to disable raw mode");

        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)
            .expect("failed to leave alternate screen");
        execute!(self.terminal.backend_mut(), DisableMouseCapture)
            .expect("failed to disable mouse capture");

        self.terminal.show_cursor().expect("failed to show cursor");
    }
}
