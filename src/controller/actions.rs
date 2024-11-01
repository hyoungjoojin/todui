use super::{key::Key, state::State};
use crate::{
    app::{
        context::{editor::EditorMode, ModalStage, SidebarStage, Stage},
        App,
    },
    model::Model,
};

impl Key {
    pub fn get_action(key: &Key) -> Box<dyn Fn((&Model, &mut App)) -> State> {
        match *key {
            Key::Quit => Box::new(|(_, _)| State::Break),
            Key::Escape => Box::new(|(_, app)| {
                if app.context_mut().modal_stage() != ModalStage::OFF {
                    app.context_mut().set_modal_stage(ModalStage::OFF);
                    return State::Continue;
                }

                if app.context_mut().stage() == Stage::EDITOR {
                    app.context_mut().set_stage(Stage::BODY);
                    return State::Continue;
                }

                if app.context_mut().stage() == Stage::BODY {
                    app.context_mut().set_stage(Stage::SIDEBAR);
                    return State::Continue;
                }

                State::Continue
            }),
            Key::Reload => Box::new(|(_, _)| State::Reload),
            Key::Enter => Box::new(|(_, app)| {
                if app.context_mut().stage() == Stage::BODY {
                    app.context_mut().set_stage(Stage::EDITOR);
                    return State::Continue;
                }

                if app.context_mut().stage() == Stage::SIDEBAR {
                    app.context_mut().set_stage(Stage::BODY);
                    return State::Continue;
                }

                State::Continue
            }),
            Key::Insert => Box::new(|(_, app)| {
                if app.context_mut().stage() == Stage::EDITOR
                    && *app.context_mut().editor_context().mode() == EditorMode::NORMAL
                {
                    app.context_mut()
                        .editor_context_mut()
                        .set_mode(EditorMode::INSERT);
                }

                State::Continue
            }),
            Key::Left => Box::new(|(_, app)| {
                if app.context_mut().stage() == Stage::EDITOR {
                    return State::Continue;
                }

                if app.context_mut().stage() == Stage::BODY {
                    return State::Continue;
                }

                let sidebar_stage = app.context_mut().sidebar_stage().previous();
                app.context_mut().set_sidebar_stage(sidebar_stage);
                State::Continue
            }),
            Key::Right => Box::new(|(_, app)| {
                if app.context_mut().stage() == Stage::EDITOR {
                    return State::Continue;
                }

                if app.context_mut().stage() == Stage::BODY {
                    return State::Continue;
                }

                let sidebar_stage = app.context_mut().sidebar_stage().next();
                app.context_mut().set_sidebar_stage(sidebar_stage);
                State::Continue
            }),
            Key::About => Box::new(|(_, app)| {
                app.context_mut().set_sidebar_stage(SidebarStage::ABOUT);
                State::Continue
            }),
            Key::Menu => Box::new(|(_, app)| {
                app.context_mut().set_sidebar_stage(SidebarStage::MENU);
                State::Continue
            }),
            Key::Projects => Box::new(|(_, app)| {
                app.context_mut().set_sidebar_stage(SidebarStage::PROJECTS);
                State::Continue
            }),
            Key::Up => Box::new(|(_, app)| {
                if app.context_mut().stage() == Stage::EDITOR {
                    let stage = app.context_mut().editor_context().stage().previous();
                    app.context_mut().editor_context_mut().set_stage(stage);
                    return State::Continue;
                }

                if app.context_mut().stage() == Stage::BODY {
                    app.scroll_tasks_up();
                    // app.scroll_tasks_down();
                    // let task_index = app.context_mut().task_index();
                    // if task_index != 0 {
                    //     app.context_mut().set_task_index(task_index - 1);
                    // }
                    app.context_mut().editor_context_mut().set_updated(true);

                    return State::Continue;
                }

                if app.context_mut().sidebar_stage() == SidebarStage::MENU {
                    let menu_stage = app.context_mut().menu_stage().previous();
                    app.context_mut().set_menu_stage(menu_stage);
                    return State::Continue;
                }

                if app.context_mut().sidebar_stage() == SidebarStage::PROJECTS {
                    let project_index = app.context_mut().project_index();
                    if project_index != 0 {
                        app.context_mut().set_project_index(project_index - 1);
                    }
                    return State::Continue;
                }

                State::Continue
            }),
            Key::Down => Box::new(|(model, app)| {
                if app.context_mut().stage() == Stage::EDITOR {
                    let stage = app.context_mut().editor_context().stage().next();
                    app.context_mut().editor_context_mut().set_stage(stage);
                    return State::Continue;
                }

                if app.context_mut().stage() == Stage::BODY {
                    app.scroll_tasks_down();
                    // let task_index = app.context_mut().task_index();
                    // if task_index + 1 != model.tasks().len() {
                    //     app.context_mut().set_task_index(task_index + 1);
                    // }
                    app.context_mut().editor_context_mut().set_updated(true);

                    return State::Continue;
                }

                if app.context_mut().sidebar_stage() == SidebarStage::MENU {
                    let menu_stage = app.context_mut().menu_stage().next();
                    app.context_mut().set_menu_stage(menu_stage);
                    return State::Continue;
                }

                if app.context_mut().sidebar_stage() == SidebarStage::PROJECTS {
                    let project_index = app.context_mut().project_index();
                    if project_index + 1 != model.projects().len() {
                        app.context_mut().set_project_index(project_index + 1);
                    }
                    return State::Continue;
                }

                State::Continue
            }),
            Key::Help => Box::new(|(_, app)| {
                if app.context_mut().modal_stage() != ModalStage::HELP {
                    app.context_mut().set_modal_stage(ModalStage::HELP);
                }

                State::Continue
            }),
            Key::Ignore => Box::new(|(_, _)| State::Continue),
        }
    }
}
