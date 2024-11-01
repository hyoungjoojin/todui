use crossterm::event::KeyCode;
use strum::EnumIter;

#[derive(PartialEq, Eq, EnumIter)]
pub enum Key {
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
    pub fn from_keycode(keycode: KeyCode) -> Key {
        match keycode {
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
        }
    }

    pub fn get_keycode(key: &Key) -> String {
        match key {
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
