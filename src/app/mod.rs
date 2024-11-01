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

pub struct App<'a> {
    context: Context,
    sidebar: Sidebar,
    pub body: Body<'a>,
    modal: Modal,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
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

    pub fn scroll_tasks_up(&mut self) {
        self.body.tasks.scroll_up();
    }

    pub fn scroll_tasks_down(&mut self) {
        self.body.tasks.scroll_down();
    }
}
