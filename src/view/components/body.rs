use chrono::{Local, NaiveDate};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{model::Model, View};

impl View {
    pub fn render_body(&self, model: &Model, frame: &mut Frame, area: Rect) {
        let panel = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)])
            .split(area);

        let tasks: Vec<Line> = model
            .tasks()
            .iter()
            .filter(|task| match *task.due() {
                Some(due) => {
                    *due.date() == NaiveDate::try_from(Local::now().naive_local()).unwrap()
                }
                None => false,
            })
            .map(|task| {
                Line::from(Span::styled(
                    format!(
                        "{} {}",
                        task.content(),
                        match *task.due() {
                            Some(due) => {
                                format!("{}", due.date())
                            }
                            None => {
                                "No due date.".to_string()
                            }
                        }
                    ),
                    Color::White,
                ))
            })
            .collect();

        let tasks_section = Paragraph::new(Text::from(tasks)).block(
            Block::default()
                .borders(Borders::ALL)
                .style(if self.context.sidebar() {
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

        frame.render_widget(tasks_section, panel[0]);
        frame.render_widget(command_section, panel[1]);
    }
}
