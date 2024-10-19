use super::Context;
use std::borrow::Cow;

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
            fields: [
                EditorField {
                    stage: EditorStage::ID,
                    title: Cow::Borrowed(" Task ID "),
                    modifiable: false,
                    value: String::new(),
                    num_lines: 3,
                },
                EditorField {
                    stage: EditorStage::CONTENT,
                    title: Cow::Borrowed(" Content "),
                    modifiable: true,
                    value: String::new(),
                    num_lines: 3,
                },
                EditorField {
                    stage: EditorStage::DESCRIPTION,
                    title: Cow::Borrowed(" Description "),
                    modifiable: true,
                    value: String::new(),
                    num_lines: 10,
                },
            ],
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

    pub fn fields(&self) -> &[EditorField; 3] {
        &self.fields
    }

    pub fn set_fields(&mut self, values: [&String; 3]) {
        for i in 0..3 {
            self.fields[i].value = values[i].clone();
        }
    }

    pub fn append_character_to_field(&mut self, stage: EditorStage, c: char) {
        match stage {
            EditorStage::ID => {
                self.fields[0].value.push(c);
            }
            EditorStage::CONTENT => {
                self.fields[1].value.push(c);
            }
            EditorStage::DESCRIPTION => {
                self.fields[2].value.push(c);
            }
        }
    }

    pub fn delete_character_from_field(&mut self, stage: EditorStage) {
        match stage {
            EditorStage::ID => {
                self.fields[0].value.pop();
            }
            EditorStage::CONTENT => {
                self.fields[1].value.pop();
            }
            EditorStage::DESCRIPTION => {
                self.fields[2].value.pop();
            }
        }
    }
}

#[derive(Clone)]
pub struct EditorField {
    pub stage: EditorStage,
    pub title: Cow<'static, str>,
    pub modifiable: bool,
    pub value: String,
    pub num_lines: u16,
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
