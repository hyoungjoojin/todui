use crate::controller::key::Key;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
    Frame,
};
use strum::IntoEnumIterator;

use super::ModalTrait;

pub struct HelpModal {}

const HELP_MODAL_TITLE: &str = " Help ";

impl HelpModal {
    pub fn new() -> HelpModal {
        HelpModal {}
    }
}

impl ModalTrait for HelpModal {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let keybindings: Vec<Line> = Key::iter()
            .map(|key| {
                if key == Key::Ignore {
                    return Line::default();
                }

                Line::default().spans(vec![
                    Span::styled(Key::get_keycode(&key).to_string(), Color::Green),
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

    fn get_vertical_layout_constraints(&self) -> Layout {
        Layout::vertical([Constraint::Percentage(60)])
    }

    fn get_horizontal_layout_constraints(&self) -> Layout {
        Layout::vertical([Constraint::Percentage(60)])
    }
}
