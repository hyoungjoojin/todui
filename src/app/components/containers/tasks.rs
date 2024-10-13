use crate::{
    app::context::{Context, MenuStage, SidebarStage, Stage},
    model::{task::Task, Model},
    utils::date::get_current_date,
};
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
        } = props;

        let height = area.height as usize - 2;
        let num_tasks = tasks.len();
        let color = if on { Color::Green } else { Color::White };

        let tasks: Vec<Line> = tasks
            .iter()
            .enumerate()
            .map(|(index, task)| {
                Line::from(Span::styled(
                    format!("{}", task.content()),
                    if index == task_index {
                        Color::Green
                    } else {
                        Color::White
                    },
                ))
            })
            .collect();

        let content = Paragraph::new(Text::from(tasks)).block(
            Block::default()
                .borders(Borders::ALL)
                .style(color)
                .title(TITLE),
        );

        if num_tasks <= height {
            frame.render_widget(content, area);
            return;
        }

        frame.render_widget(
            content.scroll(self.calculate_scroll_offset(task_index, num_tasks, height)),
            area,
        );

        let (scrollbar, mut scrollbar_state) = self.build_scrollbar(task_index, num_tasks, height);
        frame.render_stateful_widget(
            scrollbar,
            area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }

    fn build_scrollbar(
        &self,
        index: usize,
        length: usize,
        height: usize,
    ) -> (Scrollbar, ScrollbarState) {
        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .thumb_symbol("▒");

        let scrollbar_state = ScrollbarState::new(length - height / 2)
            .position(self.calculate_scrollbar_position(index, height));

        (scrollbar, scrollbar_state)
    }

    fn calculate_scroll_offset(&self, index: usize, length: usize, height: usize) -> (u16, u16) {
        let half_height = height / 2;

        let vertical_offset = if index < half_height {
            0
        } else if length - index < half_height {
            length - height
        } else {
            index - half_height
        };
        let horizontal_offset = 0;

        (vertical_offset as u16, horizontal_offset as u16)
    }

    fn calculate_scrollbar_position(&self, index: usize, height: usize) -> usize {
        let half_height = height / 2;

        if index < half_height {
            0
        } else {
            index - half_height
        }
    }
}

pub struct TasksProps<'a> {
    on: bool,
    task_index: usize,
    tasks: Vec<&'a Task>,
}

impl<'a> From<(&'a Model, &mut Context)> for TasksProps<'a> {
    fn from((model, context): (&'a Model, &mut Context)) -> TasksProps<'a> {
        let on = context.stage() != Stage::SIDEBAR;

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

        let tasks: Vec<&Task> = model.tasks().iter().filter(filter).collect();

        context.set_task_index(if tasks.len() == 0 {
            0
        } else if context.task_index() >= tasks.len() {
            tasks.len() - 1
        } else {
            context.task_index()
        });

        TasksProps {
            on,
            task_index: context.task_index(),
            tasks,
        }
    }
}
