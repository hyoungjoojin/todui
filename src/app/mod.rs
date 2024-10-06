mod components;
pub mod context;

use components::layout::{body::Body, sidebar::Sidebar};
use context::Context;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::model::Model;

pub struct App {
    context: Context,
    sidebar: Sidebar,
    body: Body,
}

impl App {
    pub fn new() -> App {
        App {
            context: Context::new(),
            sidebar: Sidebar::new(),
            body: Body::new(),
        }
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn render(&self, model: &Model, frame: &mut Frame) {
        let app = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(67)])
            .split(frame.area());

        self.sidebar.render(model, &self.context, frame, app[0]);
        self.body.render(model, &self.context, frame, app[1]);
    }
}
