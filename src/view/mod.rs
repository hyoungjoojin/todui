mod components;
pub mod context;

use crate::model::Model;
use context::ViewContext;
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    widgets::Block,
    Frame,
};

pub struct View {
    context: ViewContext,
}

impl View {
    pub fn new() -> View {
        View {
            context: ViewContext::new(),
        }
    }

    pub fn context(&self) -> &ViewContext {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut ViewContext {
        &mut self.context
    }

    pub fn render(&self, model: &Model, frame: &mut Frame) {
        let area = frame.area();

        let app = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(67)])
            .split(area);

        self.render_sidebar(model, frame, app[0]);
        self.render_body(frame, app[1]);
        self.render_modal(frame, area);
    }
}
