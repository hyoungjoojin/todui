use crate::{
    app::context::{Context, Stage},
    model::{task::Task, Model},
};
use chrono::{Local, NaiveDate};
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
        let TasksProps { on, tasks } = props;

        let color = if on { Color::Green } else { Color::White };

        let tasks: Vec<Line> = tasks
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
}

impl<'a> From<(&'a Model, &Context)> for TasksProps<'a> {
    fn from((model, context): (&'a Model, &Context)) -> TasksProps<'a> {
        let on = *context.stage() != Stage::SIDEBAR;

        TasksProps {
            on,
            tasks: model.tasks(),
        }
    }
}
