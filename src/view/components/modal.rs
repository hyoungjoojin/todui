use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Clear, Paragraph},
    Frame,
};
use strum::IntoEnumIterator;

use crate::{controller::key::Key, View};

impl View {
    pub fn render_modal(&self, frame: &mut Frame, area: Rect) {
        if !self.context().modal() {
            return;
        }

        let [area] = Layout::vertical([Constraint::Percentage(60)])
            .flex(Flex::Center)
            .areas(area);

        let [area] = Layout::horizontal([Constraint::Percentage(60)])
            .flex(Flex::Center)
            .areas(area);

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

        let modal = Paragraph::new(Text::from(keybindings))
            .block(Block::bordered().style(Style::default()).title(" Help "));

        frame.render_widget(Clear, area);
        frame.render_widget(modal, area);
    }
}
