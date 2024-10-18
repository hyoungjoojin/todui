use super::Context;

impl Context {
    pub fn editor_context(&self) -> &EditorContext {
        &self.editor_context
    }

    pub fn editor_context_mut(&mut self) -> &mut EditorContext {
        &mut self.editor_context
    }
}

pub struct EditorContext {
    updated: bool,
    stage: EditorStage,
    mode: EditorMode,
    content: String,
    description: String,
}

impl EditorContext {
    pub fn new() -> EditorContext {
        EditorContext {
            updated: true,
            stage: EditorStage::CONTENT,
            mode: EditorMode::NORMAL,
            content: String::new(),
            description: String::new(),
        }
    }

    pub fn updated(&self) -> bool {
        self.updated
    }

    pub fn set_updated(&mut self, updated: bool) {
        self.updated = updated
    }

    pub fn stage(&self) -> &EditorStage {
        &self.stage
    }

    pub fn set_stage(&mut self, stage: EditorStage) {
        self.stage = stage
    }

    pub fn mode(&self) -> &EditorMode {
        &self.mode
    }

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn content_mut(&mut self) -> &mut String {
        &mut self.content
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum EditorStage {
    CONTENT,
    DESCRIPTION,
}

impl EditorStage {
    pub fn previous(&self) -> EditorStage {
        match self {
            EditorStage::CONTENT => EditorStage::DESCRIPTION,
            EditorStage::DESCRIPTION => EditorStage::CONTENT,
        }
    }

    pub fn next(&self) -> EditorStage {
        match self {
            EditorStage::DESCRIPTION => EditorStage::CONTENT,
            EditorStage::CONTENT => EditorStage::DESCRIPTION,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum EditorMode {
    NORMAL,
    INSERT,
}
