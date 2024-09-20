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

pub struct ViewContext {
    sidebar: bool,
    sidebar_stage: SidebarStage,
}

impl ViewContext {
    pub fn new() -> ViewContext {
        ViewContext {
            sidebar: true,
            sidebar_stage: SidebarStage::ABOUT,
        }
    }

    pub fn is_sidebar_on(&self) -> bool {
        self.sidebar
    }

    pub fn get_sidebar_stage(&self) -> SidebarStage {
        self.sidebar_stage
    }

    pub fn set_sidebar_stage(&mut self, sidebar_stage: SidebarStage) {
        self.sidebar_stage = sidebar_stage
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar = !self.sidebar;
    }
}
