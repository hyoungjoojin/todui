use crate::controller::key::Key;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
    Frame,
};
use strum::IntoEnumIterator;

pub struct HelpModal {}

const HELP_MODAL_TITLE: &str = " Help ";

impl HelpModal {
    pub fn new() -> HelpModal {
        HelpModal {}
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let keybindings: Vec<Line> = Key::iter()
            .map(|key| {
                if key == Key::Ignore {
                    return Line::default();
                }

                Line::default().spans(vec![
                    Span::styled(format!("{}", Key::get_keycode(&key)), Color::Green),
                    Span::styled(format!(": {}", Key::get_description(&key)), Color::White),
                ])
            })
            .collect();

        frame.render_widget(
            Paragraph::new(Text::from(keybindings)).block(
                Block::bordered()
                    .style(Style::default())
                    .title(HELP_MODAL_TITLE),
            ),
            area,
        );
    }
}
