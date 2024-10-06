pub mod state;

use crate::{
    app::context::{Context, SidebarStage, Stage},
    model::Model,
};
use crossterm::event::{self, Event, KeyEvent};
use state::State;

pub struct Controller {}

impl Controller {
    pub fn new() -> Controller {
        Controller {}
    }

    pub fn run(&self, model: &Model, context: &mut Context) -> State {
        let key: KeyEvent = match event::read() {
            Ok(Event::Key(key)) => key,
            Ok(_) => return State::Continue,
            Err(_) => return State::Error,
        };

        if key.kind == event::KeyEventKind::Release {
            return State::Break;
        }

        match key.code {
            event::KeyCode::Char('q') => State::Break,
            event::KeyCode::Char('h') => {
                context.set_sidebar_stage(context.sidebar_stage().previous());
                State::Continue
            }
            event::KeyCode::Char('l') => {
                context.set_sidebar_stage(context.sidebar_stage().next());
                State::Continue
            }
            event::KeyCode::Char('0') => {
                context.set_sidebar_stage(SidebarStage::ABOUT);
                State::Continue
            }
            event::KeyCode::Char('1') => {
                context.set_sidebar_stage(SidebarStage::MENU);
                State::Continue
            }
            event::KeyCode::Char('2') => {
                context.set_sidebar_stage(SidebarStage::PROJECTS);
                State::Continue
            }
            event::KeyCode::Char('j') => {
                if *context.sidebar_stage() != SidebarStage::PROJECTS {
                    return State::Continue;
                }

                let project_index = context.project_index();
                if project_index + 1 != model.projects().len() {
                    context.set_project_index(project_index + 1);
                }

                State::Continue
            }
            event::KeyCode::Char('k') => {
                if *context.sidebar_stage() != SidebarStage::PROJECTS {
                    return State::Continue;
                }

                let project_index = context.project_index();
                if project_index != 0 {
                    context.set_project_index(project_index - 1);
                }

                State::Continue
            }
            event::KeyCode::Enter => {
                if *context.stage() == Stage::SIDEBAR {
                    context.set_stage(Stage::BODY);
                }

                State::Continue
            }
            event::KeyCode::Esc => {
                if *context.stage() == Stage::BODY {
                    context.set_stage(Stage::SIDEBAR);
                }

                State::Continue
            }
            _ => State::Continue,
        }
    }
}
