use crate::{
    model::Model,
    view::{context::SidebarStage, View},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

impl View {
    pub fn render_sidebar(&self, model: &Model, frame: &mut Frame, area: Rect) {
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
                        if self.context.sidebar()
                            && *self.context.sidebar_stage() == SidebarStage::ABOUT
                        {
                            Color::Green
                        } else {
                            Color::White
                        },
                    ))
                    .title(" [0] - About "),
            );

        let menu_section = Paragraph::new(Text::styled(
            "- Today",
            Style::default().fg(
                if self.context.sidebar() && *self.context.sidebar_stage() == SidebarStage::MENU {
                    Color::Green
                } else {
                    Color::White
                },
            ),
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(
                    if self.context.sidebar() && *self.context.sidebar_stage() == SidebarStage::MENU
                    {
                        Color::Green
                    } else {
                        Color::White
                    },
                )
                .title(" [1] - Menu "),
        );

        let project_index: usize = if self.context().project_index() < model.projects().len() {
            self.context().project_index()
        } else {
            0
        };

        let projects: Vec<Line> = model
            .projects()
            .iter()
            .enumerate()
            .map(|(index, project)| {
                Line::from(Span::styled(
                    format!(
                        "{} - {}",
                        if project.depth() == 0 { "" } else { "  " },
                        project.name()
                    ),
                    if index == project_index {
                        Color::Green
                    } else {
                        Color::White
                    },
                ))
            })
            .collect();

        let projects_section = Paragraph::new(Text::from(projects)).block(
            Block::default()
                .borders(Borders::ALL)
                .style(
                    if self.context.sidebar()
                        && *self.context.sidebar_stage() == SidebarStage::PROJECTS
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
