use super::editor::{Editor, EditorProps};
use crate::{
    app::context::{Context, MenuStage, SidebarStage, Stage},
    model::{task::Task, Model},
    utils::date::get_current_date,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::Color,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarState},
    Frame,
};

const TITLE: &str = " Tasks ";

pub struct Tasks {
    editor: Editor,
}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks {
            editor: Editor::new(),
        }
    }

    pub fn render(&self, props: TasksProps, frame: &mut Frame, area: Rect) {
        let TasksProps {
            on,
            editor_on,
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

        let mut selected_task: Option<&Task> = None;
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
                        selected_task = Some(task);
                        Color::Green
                    } else {
                        Color::White
                    },
                ))
            })
            .collect();

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight);
        let mut scrollbar_state = ScrollbarState::new(tasks.len()).position(task_index);

        let area = if editor_on {
            let area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(area);

            self.editor
                .render(EditorProps::new(selected_task), frame, area[1]);
            area[0]
        } else {
            area
        };

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
    editor_on: bool,
    task_index: usize,
    tasks: &'a Vec<Task>,
    filter: Box<dyn Fn(&&Task) -> bool + 'a>,
}

impl<'a> From<(&'a Model, &Context)> for TasksProps<'a> {
    fn from((model, context): (&'a Model, &Context)) -> TasksProps<'a> {
        let on = context.stage() == Stage::BODY;

        let project = model.projects().get(context.project_index());

        let filter: Box<dyn Fn(&&Task) -> bool> = match context.sidebar_stage() {
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
            editor_on: context.stage() == Stage::EDITOR,
            task_index: context.task_index(),
            tasks: model.tasks(),
            filter,
        }
    }
}
