pub struct Context {
    stage: Stage,
    sidebar_stage: SidebarStage,
    project_index: usize,
}

impl Context {
    pub fn new() -> Context {
        Context {
            stage: Stage::SIDEBAR,
            sidebar_stage: SidebarStage::ABOUT,
            project_index: 0,
        }
    }

    pub fn stage(&self) -> &Stage {
        &self.stage
    }

    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage
    }

    pub fn sidebar_stage(&self) -> &SidebarStage {
        &self.sidebar_stage
    }

    pub fn set_sidebar_stage(&mut self, sidebar_stage: SidebarStage) {
        self.sidebar_stage = sidebar_stage
    }

    pub fn project_index(&self) -> usize {
        self.project_index
    }

    pub fn set_project_index(&mut self, project_index: usize) {
        self.project_index = project_index
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Stage {
    SIDEBAR,
    BODY,
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
