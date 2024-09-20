pub mod state;

use crate::view::context::ViewContext;
use crossterm::event::{self, Event};
use state::State;

pub struct Controller {}

impl Controller {
    pub fn new() -> Controller {
        Controller {}
    }

    pub fn run(&self, view_context: &mut ViewContext) -> State {
        if let Event::Key(key) = event::read().expect("") {
            if key.kind == event::KeyEventKind::Release {
                return State::Break;
            }

            return match key.code {
                event::KeyCode::Char('q') => State::Break,
                event::KeyCode::Char('h') => {
                    view_context.set_sidebar_stage(view_context.get_sidebar_stage().previous());
                    State::Continue
                }
                event::KeyCode::Char('l') => {
                    view_context.set_sidebar_stage(view_context.get_sidebar_stage().next());
                    State::Continue
                }
                event::KeyCode::Enter => {
                    if view_context.is_sidebar_on() {
                        view_context.toggle_sidebar();
                    }
                    State::Continue
                }
                event::KeyCode::Esc => {
                    if !view_context.is_sidebar_on() {
                        view_context.toggle_sidebar();
                    }
                    State::Continue
                }
                _ => State::Continue,
            };
        }

        State::Continue
    }
}
