use crate::{
    app::context::{Context, MenuStage, SidebarStage, Stage},
    model::{task::Task, Model},
};
use chrono::{Local, NaiveDate};
use ratatui::{
    layout::{Margin, Rect},
    style::Color,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarState},
    Frame,
};

const TITLE: &str = " Tasks ";

pub struct Tasks {}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks {}
    }

    pub fn render(&self, props: TasksProps, frame: &mut Frame, area: Rect) {
        let TasksProps {
            on,
            task_index,
            tasks,
            filter,
        } = props;

        let task_index = if task_index < tasks.len() {
            task_index
        } else {
            0
        };

        let color = if on { Color::Green } else { Color::White };

        let tasks: Vec<Line> = tasks
            .iter()
            .filter(filter)
            .enumerate()
            .map(|(index, task)| {
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
                    if index == task_index {
                        Color::Green
                    } else {
                        Color::White
                    },
                ))
            })
            .collect();

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight);
        let mut scrollbar_state = ScrollbarState::new(tasks.len()).position(task_index);

        frame.render_widget(
            Paragraph::new(Text::from(tasks))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(color)
                        .title(TITLE),
                )
                .scroll((task_index as u16, 0)),
            area,
        );
        frame.render_stateful_widget(
            scrollbar,
            area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }
}

pub struct TasksProps<'a> {
    on: bool,
    task_index: usize,
    tasks: &'a Vec<Task>,
    filter: Box<dyn Fn(&&Task) -> bool>,
}

impl<'a> From<(&'a Model, &Context)> for TasksProps<'a> {
    fn from((model, context): (&'a Model, &Context)) -> TasksProps<'a> {
        let on = context.stage() != Stage::SIDEBAR;

        let filter = if context.sidebar_stage() == SidebarStage::MENU {
            match context.menu_stage() {
                MenuStage::TODAY => |task: &&Task| match *task.due() {
                    Some(due) => {
                        *due.date() == NaiveDate::try_from(Local::now().naive_local()).unwrap()
                    }
                    None => false,
                },
                MenuStage::UPCOMING => |_: &&Task| true,
            }
        } else {
            |_: &&Task| true
        };

        TasksProps {
            on,
            task_index: context.task_index(),
            tasks: model.tasks(),
            filter: Box::new(filter),
        }
    }
}
