use crossterm::event::{KeyCode, KeyEvent};
use strum::EnumIter;

#[derive(PartialEq, Eq, EnumIter, Debug)]
pub enum Key {
    Escape,
    Enter,
    CharDandD,
    CharH,
    CharI,
    CharJ,
    CharK,
    CharL,
    CharQ,
    CharShiftR,
    CharZero,
    CharOne,
    CharTwo,
    CharQuestionMark,
    Ignore,
}

impl Key {
    pub fn from_keyevent(keyevent: KeyEvent, memory: &mut Option<KeyEvent>) -> Key {
        if let Some(m) = memory.take() {
            if let KeyCode::Char('d') = m.code {
                return if let KeyCode::Char('d') = keyevent.code {
                    Key::CharDandD
                } else {
                    Key::Ignore
                };
            }
        }

        if let KeyCode::Char('d') = keyevent.code {
            memory.replace(keyevent);
            return Key::Ignore;
        }

        match keyevent.code {
            KeyCode::Char('q') => Key::CharQ,
            KeyCode::Esc => Key::Escape,
            KeyCode::Enter => Key::Enter,
            KeyCode::Char('R') => Key::CharShiftR,
            KeyCode::Char('h') => Key::CharH,
            KeyCode::Char('l') => Key::CharL,
            KeyCode::Char('k') => Key::CharK,
            KeyCode::Char('j') => Key::CharJ,
            KeyCode::Char('i') => Key::CharI,
            KeyCode::Char('0') => Key::CharZero,
            KeyCode::Char('1') => Key::CharOne,
            KeyCode::Char('2') => Key::CharTwo,
            KeyCode::Char('?') => Key::CharQuestionMark,
            _ => Key::Ignore,
        }
    }

    pub fn get_keycode(key: &Key) -> String {
        match key {
            Key::CharDandD => "d + d".to_string(),
            Key::CharQ => "q".to_string(),
            Key::Escape => "<Esc>".to_string(),
            Key::Enter => "<Enter>".to_string(),
            Key::CharI => "i".to_string(),
            Key::CharShiftR => "R".to_string(),
            Key::CharH => "h".to_string(),
            Key::CharL => "l".to_string(),
            Key::CharK => "k".to_string(),
            Key::CharJ => "j".to_string(),
            Key::CharZero => "0".to_string(),
            Key::CharOne => "1".to_string(),
            Key::CharTwo => "2".to_string(),
            Key::CharQuestionMark => "?".to_string(),
            Key::Ignore => "".to_string(),
        }
    }

    pub fn get_description(key: &Key) -> String {
        match key {
            Key::CharDandD => "Delete a task".to_string(),
            Key::CharQ => "Quit todui.".to_string(),
            Key::Escape => "Escape.".to_string(),
            Key::Enter => "Enter".to_string(),
            Key::CharI => "Enter insert mode.".to_string(),
            Key::CharShiftR => "CharShiftR.".to_string(),
            Key::CharH => "Move left.".to_string(),
            Key::CharL => "Move right.".to_string(),
            Key::CharK => "Move up.".to_string(),
            Key::CharJ => "Move down.".to_string(),
            Key::CharZero => "Set sidebar stage to about.".to_string(),
            Key::CharOne => "Set sidebar stage to menu.".to_string(),
            Key::CharTwo => "Set sidebar stage to projects.".to_string(),
            Key::CharQuestionMark => "Open help modal.".to_string(),
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
            (Key::CharDandD, None),
            (Key::Ignore, None),
            (
                Key::Ignore,
                Some(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::empty())),
            ),
        ];

        assert_eq!(input.len(), expected_output.len());

        for ((input_keyevent, mut input_memory), (target_key, target_memory)) in
            input.iter().zip(expected_output.iter())
        {
            let output_key = Key::from_keyevent(*input_keyevent, &mut input_memory);
            assert_eq!(output_key, *target_key);
            assert_eq!(input_memory, *target_memory);
        }
    }
}
