use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Frame, Terminal};
use std::{
    io::{self},
    process::exit,
};

pub struct Canvas {
    terminal: Terminal<CrosstermBackend<io::Stderr>>,
}

impl Canvas {
    pub fn new() -> Canvas {
        if let Err(error) = enable_raw_mode() {
            tracing::warn!(
                "Failed to enable raw mode due to error {error}",
                error = error
            );
        }

        if let Err(error) = execute!(io::stderr(), EnterAlternateScreen) {
            tracing::warn!(
                "Failed to enter alternate screen due to error {error}",
                error = error
            );
        }

        if let Err(error) = execute!(io::stderr(), EnableMouseCapture) {
            tracing::warn!(
                "Failed to enable mouse capture due to error {error}",
                error = error
            );
        }

        let backend = CrosstermBackend::new(io::stderr());

        let terminal = match Terminal::new(backend) {
            Ok(terminal) => terminal,
            Err(error) => {
                tracing::error!(
                    "Failed to initialize canvas due to error {error}",
                    error = error
                );
                exit(-1);
            }
        };

        Canvas { terminal }
    }

    pub fn draw<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Frame),
    {
        if let Err(error) = self.terminal.draw(f) {
            tracing::error!(
                "Failed to draw to canvas due to error {error}",
                error = error
            );
        }
    }

    pub fn clear(&mut self) {
        if let Err(error) = disable_raw_mode() {
            tracing::warn!(
                "Failed to enable raw mode due to error {error}",
                error = error
            );
        }

        if let Err(error) = execute!(self.terminal.backend_mut(), LeaveAlternateScreen) {
            tracing::warn!(
                "Failed to leave alternate screen due to error {error}",
                error = error
            );
        }

        if let Err(error) = execute!(self.terminal.backend_mut(), DisableMouseCapture) {
            tracing::warn!(
                "Failed to disable mouse capture due to error {error}",
                error = error
            );
        }

        if let Err(error) = self.terminal.show_cursor() {
            tracing::warn!("Failed to show cursor due to error {error}", error = error);
        }
    }
}
