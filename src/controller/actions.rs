use super::{key::Key, state::State};
use crate::{
    app::context::{editor::EditorMode, Context, ModalStage, SidebarStage, Stage},
    model::Model,
};

impl Key {
    pub fn get_action(key: &Key) -> Box<dyn Fn((&Model, &mut Context)) -> State> {
        match *key {
            Key::Quit => Box::new(|(_, _)| State::Break),
            Key::Escape => Box::new(|(_, context)| {
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
            }),
            Key::Enter => Box::new(|(_, context)| {
                if context.stage() == Stage::BODY {
                    context.set_stage(Stage::EDITOR);
                    return State::Continue;
                }

                if context.stage() == Stage::SIDEBAR {
                    context.set_stage(Stage::BODY);
                    return State::Continue;
                }

                State::Continue
            }),
            Key::Insert => Box::new(|(_, context)| {
                if context.stage() == Stage::EDITOR
                    && *context.editor_context().mode() == EditorMode::NORMAL
                {
                    context.editor_context_mut().set_mode(EditorMode::INSERT);
                }

                State::Continue
            }),
            Key::Left => Box::new(|(_, context)| {
                if context.stage() == Stage::EDITOR {
                    return State::Continue;
                }

                if context.stage() == Stage::BODY {
                    return State::Continue;
                }

                context.set_sidebar_stage(context.sidebar_stage().previous());
                State::Continue
            }),
            Key::Right => Box::new(|(_, context)| {
                if context.stage() == Stage::EDITOR {
                    return State::Continue;
                }

                if context.stage() == Stage::BODY {
                    return State::Continue;
                }

                context.set_sidebar_stage(context.sidebar_stage().next());
                State::Continue
            }),
            Key::About => Box::new(|(_, context)| {
                context.set_sidebar_stage(SidebarStage::ABOUT);
                State::Continue
            }),
            Key::Menu => Box::new(|(_, context)| {
                context.set_sidebar_stage(SidebarStage::MENU);
                State::Continue
            }),
            Key::Projects => Box::new(|(_, context)| {
                context.set_sidebar_stage(SidebarStage::PROJECTS);
                State::Continue
            }),
            Key::Up => Box::new(|(_, context)| {
                if context.stage() == Stage::EDITOR {
                    let stage = context.editor_context().stage().previous();
                    context.editor_context_mut().set_stage(stage);
                    return State::Continue;
                }

                if context.stage() == Stage::BODY {
                    let task_index = context.task_index();
                    if task_index != 0 {
                        context.set_task_index(task_index - 1);
                    }
                    context.editor_context_mut().set_updated(true);

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
            }),
            Key::Down => Box::new(|(model, context)| {
                if context.stage() == Stage::EDITOR {
                    let stage = context.editor_context().stage().next();
                    context.editor_context_mut().set_stage(stage);
                    return State::Continue;
                }

                if context.stage() == Stage::BODY {
                    let task_index = context.task_index();
                    if task_index + 1 != model.tasks().len() {
                        context.set_task_index(task_index + 1);
                    }
                    context.editor_context_mut().set_updated(true);

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
            }),
            Key::Help => Box::new(|(_, context)| {
                if context.modal_stage() != ModalStage::HELP {
                    context.set_modal_stage(ModalStage::HELP);
                }

                State::Continue
            }),
            Key::Ignore => Box::new(|(_, _)| State::Continue),
        }
    }
}
