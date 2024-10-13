pub mod key;
pub mod state;

use crate::{
    app::context::{Context, ModalStage, SidebarStage, Stage},
    controller::{key::Key, state::State},
    model::Model,
};
use crossterm::event::{self, Event, KeyEvent};

pub struct Controller {}

impl Controller {
    pub fn new() -> Controller {
        Controller {}
    }

    pub fn run(&self, model: &Model, context: &mut Context) -> State {
        let key: KeyEvent = match event::read() {
            Ok(Event::Key(key)) => key,
            Ok(_) => return State::Continue,
            Err(_) => return State::Error,
        };

        if key.kind == event::KeyEventKind::Release {
            return State::Break;
        }

        let key = Key::from_keycode(key.code);
        match key {
            Key::Quit => State::Break,
            Key::Escape => {
                if context.modal_stage() != ModalStage::OFF {
                    context.set_modal_stage(ModalStage::OFF);
                    return State::Continue;
                }

                if context.stage() == Stage::EDITOR {
                    context.set_stage(Stage::BODY);
                    return State::Continue;
                }

                if context.stage() == Stage::BODY {
                    context.set_stage(Stage::SIDEBAR);
                    return State::Continue;
                }

                State::Continue
            }
            Key::Enter => {
                if context.stage() == Stage::BODY {
                    context.set_stage(Stage::EDITOR);
                    return State::Continue;
                }

                if context.stage() == Stage::SIDEBAR {
                    context.set_stage(Stage::BODY);
                    return State::Continue;
                }

                State::Continue
            }
            Key::Left => {
                if context.stage() == Stage::EDITOR {
                    return State::Continue;
                }

                if context.stage() == Stage::BODY {
                    return State::Continue;
                }

                context.set_sidebar_stage(context.sidebar_stage().previous());
                State::Continue
            }
            Key::Right => {
                if context.stage() == Stage::EDITOR {
                    return State::Continue;
                }

                if context.stage() == Stage::BODY {
                    return State::Continue;
                }

                context.set_sidebar_stage(context.sidebar_stage().next());
                State::Continue
            }
            Key::About => {
                context.set_sidebar_stage(SidebarStage::ABOUT);
                State::Continue
            }
            Key::Menu => {
                context.set_sidebar_stage(SidebarStage::MENU);
                State::Continue
            }
            Key::Projects => {
                context.set_sidebar_stage(SidebarStage::PROJECTS);
                State::Continue
            }
            Key::Up => {
                if context.stage() == Stage::EDITOR {
                    return State::Continue;
                }

                if context.stage() == Stage::BODY {
                    let task_index = context.task_index();
                    if task_index != 0 {
                        context.set_task_index(task_index - 1);
                    }
                    return State::Continue;
                }

                if context.sidebar_stage() == SidebarStage::MENU {
                    context.set_menu_stage(context.menu_stage().previous());
                    return State::Continue;
                }

                if context.sidebar_stage() == SidebarStage::PROJECTS {
                    let project_index = context.project_index();
                    if project_index != 0 {
                        context.set_project_index(project_index - 1);
                    }
                    return State::Continue;
                }

                State::Continue
            }
            Key::Down => {
                if context.stage() == Stage::EDITOR {
                    return State::Continue;
                }

                if context.stage() == Stage::BODY {
                    let task_index = context.task_index();
                    if task_index + 1 != model.tasks().len() {
                        context.set_task_index(task_index + 1);
                    }
                    return State::Continue;
                }

                if context.sidebar_stage() == SidebarStage::MENU {
                    context.set_menu_stage(context.menu_stage().next());
                    return State::Continue;
                }

                if context.sidebar_stage() == SidebarStage::PROJECTS {
                    let project_index = context.project_index();
                    if project_index + 1 != model.projects().len() {
                        context.set_project_index(project_index + 1);
                    }
                    return State::Continue;
                }

                State::Continue
            }
            Key::Help => {
                if context.modal_stage() != ModalStage::HELP {
                    context.set_modal_stage(ModalStage::HELP);
                }

                State::Continue
            }
            Key::Ignore => State::Continue,
        }
    }
}
