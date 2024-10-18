pub mod editor;

use editor::EditorContext;

use crate::model::task::Task;

pub struct Context {
    stage: Stage,
    modal_stage: ModalStage,
    sidebar_stage: SidebarStage,
    menu_stage: MenuStage,
    project_index: usize,
    task_index: usize,
    selected_task: Option<Task>,
    editor_context: EditorContext,
}

impl Context {
    pub fn new() -> Context {
        Context {
            stage: Stage::SIDEBAR,
            modal_stage: ModalStage::OFF,
            sidebar_stage: SidebarStage::ABOUT,
            menu_stage: MenuStage::TODAY,
            project_index: 0,
            task_index: 0,
            selected_task: None,
            editor_context: EditorContext::new(),
        }
    }

    pub fn stage(&self) -> Stage {
        self.stage
    }

    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage
    }

    pub fn modal_stage(&self) -> ModalStage {
        self.modal_stage
    }

    pub fn set_modal_stage(&mut self, modal_stage: ModalStage) {
        self.modal_stage = modal_stage
    }

    pub fn sidebar_stage(&self) -> SidebarStage {
        self.sidebar_stage
    }

    pub fn set_sidebar_stage(&mut self, sidebar_stage: SidebarStage) {
        self.sidebar_stage = sidebar_stage
    }

    pub fn menu_stage(&self) -> MenuStage {
        self.menu_stage
    }

    pub fn set_menu_stage(&mut self, menu_stage: MenuStage) {
        self.menu_stage = menu_stage
    }

    pub fn project_index(&self) -> usize {
        self.project_index
    }

    pub fn set_project_index(&mut self, project_index: usize) {
        self.project_index = project_index
    }

    pub fn task_index(&self) -> usize {
        self.task_index
    }

    pub fn set_task_index(&mut self, task_index: usize) {
        self.task_index = task_index
    }

    pub fn selected_task(&self) -> &Option<Task> {
        &self.selected_task
    }

    pub fn set_selected_task(&mut self, selected_task: Option<Task>) {
        self.selected_task = selected_task
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Stage {
    SIDEBAR,
    BODY,
    EDITOR,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ModalStage {
    OFF,
    HELP,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SidebarStage {
    ABOUT,
    MENU,
    PROJECTS,
}

impl SidebarStage {
    pub fn previous(&self) -> SidebarStage {
        match self {
            SidebarStage::ABOUT => SidebarStage::PROJECTS,
            SidebarStage::MENU => SidebarStage::ABOUT,
            SidebarStage::PROJECTS => SidebarStage::MENU,
        }
    }

    pub fn next(&self) -> SidebarStage {
        match self {
            SidebarStage::ABOUT => SidebarStage::MENU,
            SidebarStage::MENU => SidebarStage::PROJECTS,
            SidebarStage::PROJECTS => SidebarStage::ABOUT,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MenuStage {
    TODAY,
    UPCOMING,
}

impl MenuStage {
    pub fn previous(&self) -> MenuStage {
        match self {
            MenuStage::TODAY => MenuStage::UPCOMING,
            MenuStage::UPCOMING => MenuStage::TODAY,
        }
    }

    pub fn next(&self) -> MenuStage {
        match self {
            MenuStage::UPCOMING => MenuStage::TODAY,
            MenuStage::TODAY => MenuStage::UPCOMING,
        }
    }
}
