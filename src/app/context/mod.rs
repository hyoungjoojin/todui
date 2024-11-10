pub mod editor;

use editor::EditorContext;

use crate::model::task::Task;

pub struct Context {
    stage: Stage,
    modal_stage: ModalStage,
    sidebar_stage: SidebarStage,
    menu_stage: MenuStage,
    project_index: usize,
    selected_task: Option<Task>,
    editor_context: EditorContext,
}

impl Context {
    pub fn new() -> Context {
        Context {
            stage: Stage::Sidebar,
            modal_stage: ModalStage::Off,
            sidebar_stage: SidebarStage::About,
            menu_stage: MenuStage::Today,
            project_index: 0,
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

    pub fn selected_task(&self) -> &Option<Task> {
        &self.selected_task
    }

    pub fn set_selected_task(&mut self, selected_task: Option<Task>) {
        self.selected_task = selected_task
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Stage {
    Sidebar,
    Body,
    Editor,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ModalStage {
    Help,
    Project,
    Off,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SidebarStage {
    About,
    Menu,
    Projects,
}

impl SidebarStage {
    pub fn previous(&self) -> SidebarStage {
        match self {
            SidebarStage::About => SidebarStage::Projects,
            SidebarStage::Menu => SidebarStage::About,
            SidebarStage::Projects => SidebarStage::Menu,
        }
    }

    pub fn next(&self) -> SidebarStage {
        match self {
            SidebarStage::About => SidebarStage::Menu,
            SidebarStage::Menu => SidebarStage::Projects,
            SidebarStage::Projects => SidebarStage::About,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MenuStage {
    Today,
    Upcoming,
}

impl MenuStage {
    pub fn previous(&self) -> MenuStage {
        match self {
            MenuStage::Today => MenuStage::Upcoming,
            MenuStage::Upcoming => MenuStage::Today,
        }
    }

    pub fn next(&self) -> MenuStage {
        match self {
            MenuStage::Upcoming => MenuStage::Today,
            MenuStage::Today => MenuStage::Upcoming,
        }
    }
}
