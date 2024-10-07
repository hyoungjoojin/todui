use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{
    app::{
        components::containers::{about::About, menu::Menu, projects::Projects},
        context::Context,
    },
    model::Model,
};

pub struct Sidebar {
    about: About,
    menu: Menu,
    projects: Projects,
}

impl Sidebar {
    pub fn new() -> Sidebar {
        Sidebar {
            about: About::new(),
            menu: Menu::new(),
            projects: Projects::new(),
        }
    }

    pub fn render(&self, model: &Model, context: &Context, frame: &mut Frame, area: Rect) {
        let panel = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(3),
            ])
            .split(area);

        let state = (model, context);
        self.about.render(state.into(), frame, panel[0]);
        self.menu.render(state.into(), frame, panel[1]);
        self.projects.render(state.into(), frame, panel[2]);
    }
}
