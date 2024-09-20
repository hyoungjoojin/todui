mod body;
pub mod context;
mod sidebar;

use context::ViewContext;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub struct View {
    pub context: ViewContext,
}

impl View {
    pub fn new() -> View {
        View {
            context: ViewContext::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let app = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(67)])
            .split(frame.area());

        self.render_sidebar(frame, app[0]);
        self.render_body(frame, app[1]);
    }
}
