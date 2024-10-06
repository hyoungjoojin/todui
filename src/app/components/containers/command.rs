use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Command {}

impl Command {
    pub fn new() -> Command {
        Command {}
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new(Text::styled("", Style::default().fg(Color::White))).block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Color::White)
                    .title(" Command "),
            ),
            area,
        );
    }
}
