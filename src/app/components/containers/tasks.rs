use crate::{
    app::context::{Context, MenuStage, SidebarStage, Stage},
    model::{task::Task, Model},
    utils::date::get_current_date,
};
use ratatui::{
    layout::Rect,
    style::Color,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const TITLE: &str = " Tasks ";

pub struct Tasks {}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks {}
    }

    pub fn render(&self, props: TasksProps, frame: &mut Frame, area: Rect) {
        let TasksProps { on, tasks, filter } = props;

        let color = if on { Color::Green } else { Color::White };

        let tasks: Vec<Line> = tasks
            .iter()
            .filter(filter)
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

        frame.render_widget(
            Paragraph::new(Text::from(tasks)).block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(color)
                    .title(TITLE),
            ),
            area,
        );
    }
}

pub struct TasksProps<'a> {
    on: bool,
    tasks: &'a Vec<Task>,
    filter: Box<dyn Fn(&&Task) -> bool + 'a>,
}

impl<'a> From<(&'a Model, &Context)> for TasksProps<'a> {
    fn from((model, context): (&'a Model, &Context)) -> TasksProps<'a> {
        let on = *context.stage() != Stage::SIDEBAR;

        let project = model.projects().get(context.project_index());

        let filter: Box<dyn Fn(&&Task) -> bool> = match *context.sidebar_stage() {
            SidebarStage::ABOUT => Box::new(|_: &&Task| false),
            SidebarStage::MENU => match context.menu_stage() {
                MenuStage::TODAY => Box::new(|task: &&Task| match *task.due() {
                    Some(due) => *due.date() == get_current_date(),
                    None => false,
                }),
                MenuStage::UPCOMING => Box::new(|_: &&Task| true),
            },
            SidebarStage::PROJECTS => Box::new(move |task: &&Task| match project {
                Some(project) => task.project_id() == project.id(),
                None => false,
            }),
        };

        TasksProps {
            on,
            tasks: model.tasks(),
            filter,
        }
    }
}
