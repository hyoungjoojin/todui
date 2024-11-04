use crossterm::event::{KeyCode, KeyEvent};
use strum::EnumIter;

#[derive(PartialEq, Eq, EnumIter, Debug)]
pub enum Key {
    CharDandD,

    Quit,
    Escape,
    Enter,
    Reload,

    Insert,

    Left,
    Right,
    Up,
    Down,

    About,
    Menu,
    Projects,

    Help,

    Ignore,
}

impl Key {
    /// Creates a Key instance from a given KeyEvent.
    ///
    /// This function takes a KeyEvent of the current key input event and returns a Key
    /// instance. Since events where a key is double tapped can exist, a memory variable
    /// is used. This function also returns a boolean value alongside the Key variable,
    /// that indicates whether or not the memory should be updated.
    ///
    /// ## Parameters
    /// - keyevent (crossterm::event::KeyEvent): The current key input event.
    /// - memory (Option<KeyEvent>): The previous key input event.
    ///
    /// ## Returns
    /// - A tuple containing:
    ///   - 'Key': The created Key instance.
    ///   - 'bool': A boolean value indicating whether to update the memory. Returns true
    ///             if the memory should be updated and false otherwise.
    pub fn from_keyevent(keyevent: KeyEvent, memory: Option<KeyEvent>) -> (Key, bool) {
        if let Some(memory) = memory {
            match memory.code {
                KeyCode::Char('d') => match keyevent.code {
                    KeyCode::Char('d') => return (Key::CharDandD, false),
                    _ => return (Key::Ignore, false),
                },
                _ => {}
            }
        }

        match keyevent.code {
            KeyCode::Char('d') => return (Key::Ignore, true),
            _ => {}
        }

        (
            match keyevent.code {
                KeyCode::Char('q') => Key::Quit,
                KeyCode::Esc => Key::Escape,
                KeyCode::Enter => Key::Enter,
                KeyCode::Char('r') => Key::Reload,
                KeyCode::Char('h') => Key::Left,
                KeyCode::Char('l') => Key::Right,
                KeyCode::Char('k') => Key::Up,
                KeyCode::Char('j') => Key::Down,
                KeyCode::Char('i') => Key::Insert,
                KeyCode::Char('0') => Key::About,
                KeyCode::Char('1') => Key::Menu,
                KeyCode::Char('2') => Key::Projects,
                KeyCode::Char('?') => Key::Help,
                _ => Key::Ignore,
            },
            false,
        )
    }

    pub fn get_keycode(key: &Key) -> String {
        match key {
            Key::CharDandD => "d + d".to_string(),
            Key::Quit => "q".to_string(),
            Key::Escape => "<Esc>".to_string(),
            Key::Enter => "<Enter>".to_string(),
            Key::Insert => "i".to_string(),
            Key::Reload => "r".to_string(),
            Key::Left => "l".to_string(),
            Key::Right => "r".to_string(),
            Key::Up => "k".to_string(),
            Key::Down => "j".to_string(),
            Key::About => "0".to_string(),
            Key::Menu => "1".to_string(),
            Key::Projects => "2".to_string(),
            Key::Help => "?".to_string(),
            Key::Ignore => "".to_string(),
        }
    }

    pub fn get_description(key: &Key) -> String {
        match key {
            Key::CharDandD => "Delete a task".to_string(),
            Key::Quit => "Quit todui.".to_string(),
            Key::Escape => "Escape.".to_string(),
            Key::Enter => "Enter".to_string(),
            Key::Insert => "Enter insert mode.".to_string(),
            Key::Reload => "Reload.".to_string(),
            Key::Left => "Move left.".to_string(),
            Key::Right => "Move right.".to_string(),
            Key::Up => "Move up.".to_string(),
            Key::Down => "Move down.".to_string(),
            Key::About => "Set sidebar stage to about.".to_string(),
            Key::Menu => "Set sidebar stage to menu.".to_string(),
            Key::Projects => "Set sidebar stage to projects.".to_string(),
            Key::Help => "Open help modal.".to_string(),
            Key::Ignore => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Key;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_from_keyevent() {
        let input = [
            // Pressing double ds.
            (
                KeyEvent::new(KeyCode::Char('d'), KeyModifiers::empty()),
                Some(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::empty())),
            ),
            // Pressing one d and another key.
            (
                KeyEvent::new(KeyCode::Char('e'), KeyModifiers::empty()),
                Some(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::empty())),
            ),
            // Pressing one d.
            (
                KeyEvent::new(KeyCode::Char('d'), KeyModifiers::empty()),
                None,
            ),
        ];
        let expected_output = [
            (Key::CharDandD, false),
            (Key::Ignore, false),
            (Key::Ignore, true),
        ];

        assert_eq!(input.len(), expected_output.len());

        for ((keyevent, memory), (key, set_memory)) in input.iter().zip(expected_output.iter()) {
            let (output_key, output_set_memory) = Key::from_keyevent(*keyevent, *memory);
            assert_eq!(*key, output_key);
            assert_eq!(*set_memory, output_set_memory);
        }
    }
}
