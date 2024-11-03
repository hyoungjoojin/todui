mod actions;
pub mod key;
pub mod state;

use crate::{
    app::{
        context::{editor::EditorMode, Stage},
        App,
    },
    controller::{key::Key, state::State},
    model::Model,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent};

pub struct Controller {
    memory: Option<KeyEvent>,
}

impl Controller {
    pub fn new() -> Controller {
        Controller { memory: None }
    }

    pub fn run(&mut self, model: &Model, app: &mut App) -> State {
        let keyevent: KeyEvent = match event::read() {
            Ok(Event::Key(key)) => key,
            Ok(_) => return State::Continue,
            Err(_) => return State::Error,
        };

        let context = app.context_mut();

        if context.stage() == Stage::Editor
            && *context.editor_context().mode() == EditorMode::Insert
        {
            if let KeyCode::Char(c) = keyevent.code {
                let stage = *context.editor_context().stage();
                context
                    .editor_context_mut()
                    .append_character_to_field(stage, c);
            };

            if keyevent.code == KeyCode::Backspace {
                let stage = *context.editor_context().stage();
                context
                    .editor_context_mut()
                    .delete_character_from_field(stage);
            };

            if keyevent.code == KeyCode::Esc {
                context.editor_context_mut().set_mode(EditorMode::Normal);
            }

            return State::Continue;
        }

        let (key, set_memory) = Key::from_keyevent(keyevent, self.memory);
        self.memory = if set_memory { Some(keyevent) } else { None };

        Key::get_action(&key)((model, app))
    }
}
