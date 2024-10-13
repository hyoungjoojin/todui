mod components;
pub mod context;

use crate::{
    app::{
        components::layout::{body::Body, sidebar::Sidebar},
        context::Context,
    },
    model::Model,
};
use components::layout::modal::Modal;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub struct App {
    context: Context,
    sidebar: Sidebar,
    body: Body,
    modal: Modal,
}

impl App {
    pub fn new() -> App {
        App {
            context: Context::new(),
            sidebar: Sidebar::new(),
            body: Body::new(),
            modal: Modal::new(),
        }
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn render(&mut self, model: &Model, frame: &mut Frame) {
        let area = frame.area();

        let app = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(67)])
            .split(area);

        self.sidebar.render(model, &self.context, frame, app[0]);
        self.body.render(model, &mut self.context, frame, app[1]);
        self.modal.render(&self.context, frame, area);
    }
}
