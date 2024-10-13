use crate::{
    app::{
        components::containers::{command::Command, tasks::Tasks},
        context::Context,
    },
    model::Model,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub struct Body {
    tasks: Tasks,
    command: Command,
}

impl Body {
    pub fn new() -> Body {
        Body {
            tasks: Tasks::new(),
            command: Command::new(),
        }
    }

    pub fn render(&self, model: &Model, context: &mut Context, frame: &mut Frame, area: Rect) {
        let panel = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)])
            .split(area);

        self.tasks.render((model, context).into(), frame, panel[0]);
        self.command.render(frame, panel[1]);
    }
}
