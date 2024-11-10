use super::{key::Key, state::State};
use crate::{
    app::{
        context::{editor::EditorMode, ModalStage, SidebarStage, Stage},
        App,
    },
    model::Model,
};

type ActionFn = Box<dyn Fn((&Model, &mut App)) -> State>;

impl Key {
    pub fn get_action(key: &Key) -> ActionFn {
        match *key {
            Key::Escape => Box::new(|(_, app)| {
                let context = app.context_mut();

                if context.modal_stage() != ModalStage::Off {
                    context.set_modal_stage(ModalStage::Off);
                    return State::Continue;
                }

                if context.stage() == Stage::Editor {
                    context.set_stage(Stage::Body);
                    return State::Continue;
                }

                if context.stage() == Stage::Body {
                    context.set_stage(Stage::Sidebar);
                    return State::Continue;
                }

                State::Continue
            }),
            Key::Enter => Box::new(|(_, app)| {
                let context = app.context_mut();

                if context.stage() == Stage::Editor {
                    return State::PostTask;
                }

                if context.stage() == Stage::Body {
                    context.set_stage(Stage::Editor);
                    return State::Continue;
                }

                if context.stage() == Stage::Sidebar {
                    context.set_stage(Stage::Body);
                    return State::Continue;
                }

                State::Continue
            }),
            Key::CharDandD => Box::new(|(_, app)| match app.context_mut().selected_task() {
                Some(task) => State::DeleteTask(task.id().clone()),
                None => State::Continue,
            }),
            Key::CharH => Box::new(|(_, app)| {
                let context = app.context_mut();

                if context.stage() == Stage::Editor {
                    return State::Continue;
                }

                if context.stage() == Stage::Body {
                    return State::Continue;
                }

                let sidebar_stage = context.sidebar_stage().previous();
                context.set_sidebar_stage(sidebar_stage);
                State::Continue
            }),
            Key::CharI => Box::new(|(_, app)| {
                let context = app.context_mut();

                if context.stage() == Stage::Editor
                    && *context.editor_context().mode() == EditorMode::Normal
                {
                    context.editor_context_mut().set_mode(EditorMode::Insert);
                }

                State::Continue
            }),
            Key::CharJ => Box::new(|(model, app)| {
                let context = app.context_mut();

                if context.stage() == Stage::Editor {
                    let stage = context.editor_context().stage().next();
                    context.editor_context_mut().set_stage(stage);
                    return State::Continue;
                }

                if context.stage() == Stage::Body {
                    context.editor_context_mut().set_updated(true);
                    app.scroll_tasks_down();

                    return State::Continue;
                }

                if context.sidebar_stage() == SidebarStage::Menu {
                    let menu_stage = context.menu_stage().next();
                    context.set_menu_stage(menu_stage);
                    return State::Continue;
                }

                if context.sidebar_stage() == SidebarStage::Projects {
                    let project_index = context.project_index();
                    if project_index + 1 != model.projects().len() {
                        context.set_project_index(project_index + 1);
                    }
                    return State::Continue;
                }

                State::Continue
            }),
            Key::CharK => Box::new(|(_, app)| {
                let context = app.context_mut();

                if context.stage() == Stage::Editor {
                    let stage = context.editor_context().stage().previous();
                    context.editor_context_mut().set_stage(stage);
                    return State::Continue;
                }

                if context.stage() == Stage::Body {
                    context.editor_context_mut().set_updated(true);
                    app.scroll_tasks_up();

                    return State::Continue;
                }

                if context.sidebar_stage() == SidebarStage::Menu {
                    let menu_stage = context.menu_stage().previous();
                    context.set_menu_stage(menu_stage);
                    return State::Continue;
                }

                if context.sidebar_stage() == SidebarStage::Projects {
                    let project_index = context.project_index();
                    if project_index != 0 {
                        context.set_project_index(project_index - 1);
                    }
                    return State::Continue;
                }

                State::Continue
            }),
            Key::CharL => Box::new(|(_, app)| {
                let context = app.context_mut();

                if context.stage() == Stage::Editor {
                    return State::Continue;
                }

                if context.stage() == Stage::Body {
                    return State::Continue;
                }

                let sidebar_stage = context.sidebar_stage().next();
                context.set_sidebar_stage(sidebar_stage);
                State::Continue
            }),
            Key::CharQ => Box::new(|(_, _)| State::Break),
            Key::CharR => Box::new(|(_, app)| {
                let context = app.context_mut();

                if context.stage() == Stage::Sidebar
                    && context.sidebar_stage() == SidebarStage::Projects
                    && context.modal_stage() != ModalStage::Project
                {
                    context.set_modal_stage(ModalStage::Project);
                }

                State::Continue
            }),
            Key::CharShiftR => Box::new(|(_, _)| State::Reload),
            Key::CharZero => Box::new(|(_, app)| {
                let context = app.context_mut();

                context.set_sidebar_stage(SidebarStage::About);
                State::Continue
            }),
            Key::CharOne => Box::new(|(_, app)| {
                let context = app.context_mut();

                context.set_sidebar_stage(SidebarStage::Menu);
                State::Continue
            }),
            Key::CharTwo => Box::new(|(_, app)| {
                let context = app.context_mut();

                context.set_sidebar_stage(SidebarStage::Projects);
                State::Continue
            }),
            Key::CharQuestionMark => Box::new(|(_, app)| {
                let context = app.context_mut();

                if context.modal_stage() != ModalStage::Help {
                    context.set_modal_stage(ModalStage::Help);
                }

                State::Continue
            }),
            Key::Ignore => Box::new(|(_, _)| State::Continue),
        }
    }
}
