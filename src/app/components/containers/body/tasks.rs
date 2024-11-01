use crate::{
    app::context::{Context, MenuStage, SidebarStage, Stage},
    model::{task::Task, Model},
    utils::date::get_current_date,
};
use ratatui::{
    layout::{Constraint, Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Scrollbar, ScrollbarState, Table, TableState},
    Frame,
};

const TITLE: &str = " Tasks ";

pub struct Tasks<'a> {
    table_state: TableState,
    scrollbar: Scrollbar<'a>,
    scrollbar_state: ScrollbarState,
}

impl<'a> Tasks<'a> {
    pub fn new() -> Tasks<'a> {
        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .thumb_symbol("▒");

        let scrollbar_state = ScrollbarState::new(0).position(0);

        Tasks {
            table_state: TableState::default().with_selected(1),
            scrollbar,
            scrollbar_state,
        }
    }

    pub fn render(&mut self, props: TasksProps, frame: &mut Frame, area: Rect) -> TasksReturnProps {
        let TasksProps { on, tasks } = props;

        self.scrollbar_state = self.scrollbar_state.content_length(tasks.len());

        let height = area.height as usize - 2;
        let num_tasks = tasks.len();
        let color = if on { Color::Green } else { Color::White };

        let header = ["Project", "Content", "Due Date", "Priority"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(Style::default().fg(Color::White).bg(Color::Black));

        let table = Table::new(
            tasks
                .iter()
                .map(|task| Row::from(task).style(Style::new().fg(Color::White))),
            [
                Constraint::Length(10),
                Constraint::Max(area.width * 2 / 3),
                Constraint::Length(11),
                Constraint::Length(8),
            ],
        )
        .header(header)
        .highlight_style(Style::new().fg(Color::Green))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(color)
                .title(TITLE),
        );

        frame.render_stateful_widget(table, area, &mut self.table_state);

        if height < num_tasks {
            match self.table_state.selected() {
                Some(index) => {
                    self.scrollbar_state = self.scrollbar_state.position(index);
                }
                None => {}
            }

            frame.render_stateful_widget(
                self.scrollbar.clone(),
                area.inner(Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &mut self.scrollbar_state,
            );
        }

        return TasksReturnProps {
            selected_task: match self.table_state.selected() {
                Some(index) => Some(tasks[index].clone()),
                None => None,
            },
        };
    }

    pub fn scroll_down(&mut self) {
        self.table_state.scroll_down_by(1);
    }

    pub fn scroll_up(&mut self) {
        self.table_state.scroll_up_by(1);
    }
}

pub struct TasksProps<'a> {
    on: bool,
    tasks: Vec<&'a Task>,
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
                MenuStage::UPCOMING => Box::new(|task: &&Task| match *task.due() {
                    Some(_) => true,
                    None => false,
                }),
            },
            SidebarStage::PROJECTS => Box::new(move |task: &&Task| match project {
                Some(project) => task.project_id() == project.id(),
                None => false,
            }),
        };

        let mut tasks: Vec<&Task> = model.tasks().iter().filter(filter).collect();

        if context.sidebar_stage() == SidebarStage::MENU
            && context.menu_stage() == MenuStage::UPCOMING
        {
            tasks.sort_by(|task1, task2| {
                task1.due().unwrap().date().cmp(task2.due().unwrap().date())
            });
        }

        TasksProps { on, tasks }
    }
}

pub struct TasksReturnProps {
    pub selected_task: Option<Task>,
}
