use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::View;

impl View {
    pub fn render_body(&self, frame: &mut Frame, area: Rect) {
        let panel = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)])
            .split(area);

        let body_section = Paragraph::new(Text::styled("", Style::default().fg(Color::White)))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(if self.context.is_sidebar_on() {
                        Color::White
                    } else {
                        Color::Green
                    })
                    .title(" Tasks "),
            );

        let command_section = Paragraph::new(Text::styled("", Style::default().fg(Color::White)))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default())
                    .title(" Command "),
            );

        frame.render_widget(body_section, panel[0]);
        frame.render_widget(command_section, panel[1]);
    }
}
