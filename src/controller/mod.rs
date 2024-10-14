mod actions;
pub mod key;
pub mod state;

use crate::{
    app::context::Context,
    controller::{key::Key, state::State},
    model::Model,
};
use crossterm::event::{self, Event, KeyEvent};

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

        let key = Key::from_keycode(key.code);
        Key::get_action(&key)((model, context))
    }
}
