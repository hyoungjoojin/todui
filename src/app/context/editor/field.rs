use std::borrow::Cow;

use super::{EditorContext, EditorStage};

pub const NUM_EDITOR_FIELDS: usize = 3;

#[derive(Clone)]
pub struct EditorField {
    pub stage: EditorStage,
    pub title: Cow<'static, str>,
    pub modifiable: bool,
    pub value: String,
    pub num_lines: u16,
}

impl EditorField {
    pub fn build_initial_fields() -> [EditorField; NUM_EDITOR_FIELDS] {
        [
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
        ]
    }

    pub fn get_field_index(stage: EditorStage) -> usize {
        match stage {
            EditorStage::ID => 0,
            EditorStage::CONTENT => 1,
            EditorStage::DESCRIPTION => 2,
        }
    }
}

impl EditorContext {
    pub fn fields(&self) -> &[EditorField; NUM_EDITOR_FIELDS] {
        &self.fields
    }

    pub fn set_fields(&mut self, values: [&String; NUM_EDITOR_FIELDS]) {
        for i in 0..values.len() {
            self.fields[i].value = values[i].clone();
        }
    }

    pub fn append_character_to_field(&mut self, stage: EditorStage, c: char) {
        self.fields[EditorField::get_field_index(stage)]
            .value
            .push(c);
    }

    pub fn delete_character_from_field(&mut self, stage: EditorStage) {
        self.fields[EditorField::get_field_index(stage)].value.pop();
    }
}
