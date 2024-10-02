use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Style,
    text::Text,
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::View;

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

        let modal = Paragraph::new(Text::styled("", Style::default()))
            .block(Block::bordered().style(Style::default()).title(" Help "));

        frame.render_widget(Clear, area);
        frame.render_widget(modal, area);
    }
}
