mod actions;
pub mod key;
pub mod state;

use crate::{
    app::context::{editor::EditorMode, Context, Stage},
    controller::{key::Key, state::State},
    model::Model,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent};

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

        if context.stage() == Stage::EDITOR
            && *context.editor_context().mode() == EditorMode::INSERT
        {
            if let KeyCode::Char(c) = key.code {
                let stage = *context.editor_context().stage();
                context
                    .editor_context_mut()
                    .append_character_to_field(stage, c);
            };

            if key.code == KeyCode::Backspace {
                let stage = *context.editor_context().stage();
                context
                    .editor_context_mut()
                    .delete_character_from_field(stage);
            };

            if key.code == KeyCode::Esc {
                context.editor_context_mut().set_mode(EditorMode::NORMAL);
            }

            return State::Continue;
        }

        let key = Key::from_keycode(key.code);
        Key::get_action(&key)((model, context))
    }
}
