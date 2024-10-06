use crate::{
    app::context::{Context, SidebarStage, Stage},
    model::{project::Project, Model},
};
use ratatui::{
    layout::Rect,
    style::Color,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Projects {}

impl Projects {
    pub fn new() -> Projects {
        Projects {}
    }

    pub fn render(&self, props: ProjectsProps, frame: &mut Frame, area: Rect) {
        let ProjectsProps {
            on,
            project_index,
            projects,
        } = props;

        let project_index = if project_index < projects.len() {
            project_index
        } else {
            0
        };

        let color = if on { Color::Green } else { Color::White };

        let projects: Vec<Line> = projects
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

        frame.render_widget(
            Paragraph::new(Text::from(projects)).block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(color)
                    .title(" [2] - Projects "),
            ),
            area,
        );
    }
}

pub struct ProjectsProps<'a> {
    on: bool,
    project_index: usize,
    projects: &'a Vec<Project>,
}

impl<'a> From<(&'a Model, &Context)> for ProjectsProps<'a> {
    fn from((model, context): (&'a Model, &Context)) -> ProjectsProps<'a> {
        let on = *context.stage() == Stage::SIDEBAR
            && *context.sidebar_stage() == SidebarStage::PROJECTS;

        ProjectsProps {
            on,
            project_index: context.project_index(),
            projects: model.projects(),
        }
    }
}
