use field::EditorField;

use super::Context;
pub mod field;

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
    fields: [EditorField; 3],
}

impl EditorContext {
    pub fn new() -> EditorContext {
        EditorContext {
            updated: true,
            stage: EditorStage::CONTENT,
            mode: EditorMode::NORMAL,
            fields: EditorField::build_initial_fields(),
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
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum EditorStage {
    ID,
    CONTENT,
    DESCRIPTION,
}

impl EditorStage {
    pub fn previous(&self) -> EditorStage {
        match self {
            EditorStage::ID => EditorStage::ID,
            EditorStage::CONTENT => EditorStage::DESCRIPTION,
            EditorStage::DESCRIPTION => EditorStage::CONTENT,
        }
    }

    pub fn next(&self) -> EditorStage {
        match self {
            EditorStage::ID => EditorStage::ID,
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
