use std::borrow::Borrow;

use crate::{
    app::{
        components::containers::body::{
            about::About,
            command::Command,
            editor::Editor,
            tasks::{Tasks, TasksReturnProps},
        },
        context::{Context, SidebarStage, Stage},
    },
    model::Model,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub struct Body {
    about: About,
    tasks: Tasks,
    editor: Editor,
    command: Command,
}

impl Body {
    pub fn new() -> Body {
        Body {
            about: About::new(),
            tasks: Tasks::new(),
            command: Command::new(),
            editor: Editor::new(),
        }
    }

    pub fn render(&self, model: &Model, context: &mut Context, frame: &mut Frame, area: Rect) {
        let panel = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)])
            .split(area);

        self.command.render(frame, panel[1]);

        if context.sidebar_stage() == SidebarStage::ABOUT {
            self.about.render(frame, panel[0]);
            return;
        }

        if context.stage() != Stage::EDITOR {
            self.tasks
                .render((model, context.borrow()).into(), frame, panel[0]);
            return;
        }

        let area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(panel[0]);

        let TasksReturnProps {
            task_index,
            selected_task,
        } = self
            .tasks
            .render((model, context.borrow()).into(), frame, area[0]);

        context.set_task_index(task_index);
        context.set_selected_task(selected_task);

        if let Some(task) = context.selected_task().clone() {
            if context.editor_context().updated() {
                context.editor_context_mut().set_fields([
                    &task.id(),
                    &task.content(),
                    &task.description(),
                ]);

                context.editor_context_mut().set_updated(false);
            }

            self.editor
                .render((model, context.borrow()).into(), frame, area[1]);
        }
    }
}
