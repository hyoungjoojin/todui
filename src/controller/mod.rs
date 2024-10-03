pub mod key;
pub mod state;

use crate::{
    model::Model,
    view::context::{SidebarStage, ViewContext},
};
use crossterm::event::{self, Event, KeyEvent};
use key::Key;
use state::State;

pub struct Controller {}

impl Controller {
    pub fn new() -> Controller {
        Controller {}
    }

    pub fn run(&self, model: &Model, view_context: &mut ViewContext) -> State {
        let key: KeyEvent = match event::read() {
            Ok(Event::Key(key)) => key,
            Ok(_) => return State::Continue,
            Err(_) => return State::Error,
        };

        if key.kind == event::KeyEventKind::Release {
            return State::Break;
        }

        let key = Key::from_keycode(key.code);
        match key {
            Key::Quit => State::Break,
            Key::Escape => {
                if view_context.modal() {
                    view_context.toggle_modal();
                    return State::Continue;
                }

                if !view_context.sidebar() {
                    view_context.toggle_sidebar();
                }

                State::Continue
            }
            Key::Enter => {
                if view_context.sidebar() {
                    view_context.toggle_sidebar();
                }

                State::Continue
            }
            Key::Left => {
                view_context.set_sidebar_stage(view_context.sidebar_stage().previous());
                State::Continue
            }
            Key::Right => {
                view_context.set_sidebar_stage(view_context.sidebar_stage().next());
                State::Continue
            }
            Key::Up => {
                if *view_context.sidebar_stage() != SidebarStage::PROJECTS {
                    return State::Continue;
                }

                let project_index = view_context.project_index();
                if project_index != 0 {
                    view_context.set_project_index(project_index - 1);
                }

                State::Continue
            }
            Key::Down => {
                if *view_context.sidebar_stage() != SidebarStage::PROJECTS {
                    return State::Continue;
                }

                let project_index = view_context.project_index();
                if project_index + 1 != model.projects().len() {
                    view_context.set_project_index(project_index + 1);
                }

                State::Continue
            }
            Key::Help => {
                if !view_context.modal() {
                    view_context.toggle_modal();
                }

                State::Continue
            }
            Key::Ignore => State::Continue,
        }
    }
}
