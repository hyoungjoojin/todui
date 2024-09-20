use super::context::SidebarStage;
use crate::View;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

impl View {
    pub fn render_sidebar(&self, frame: &mut Frame, area: Rect) {
        let panel = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(3),
            ])
            .split(area);

        let about_section = Paragraph::new(Text::styled("", Style::default().fg(Color::White)))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(
                        if self.context.is_sidebar_on()
                            && self.context.get_sidebar_stage() == SidebarStage::ABOUT
                        {
                            Color::Green
                        } else {
                            Color::White
                        },
                    ))
                    .title(" [0] - About "),
            );

        let menu_section = Paragraph::new(Text::styled("", Style::default().fg(Color::White)))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(
                        if self.context.is_sidebar_on()
                            && self.context.get_sidebar_stage() == SidebarStage::MENU
                        {
                            Color::Green
                        } else {
                            Color::White
                        },
                    )
                    .title(" [1] - Menu "),
            );

        let projects_section = Paragraph::new(Text::styled("", Style::default().fg(Color::White)))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(
                        if self.context.is_sidebar_on()
                            && self.context.get_sidebar_stage() == SidebarStage::PROJECTS
                        {
                            Color::Green
                        } else {
                            Color::White
                        },
                    )
                    .title(" [2] - Projects "),
            );

        frame.render_widget(about_section, panel[0]);
        frame.render_widget(menu_section, panel[1]);
        frame.render_widget(projects_section, panel[2]);
    }
}
