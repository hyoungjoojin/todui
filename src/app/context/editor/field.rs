use super::{EditorContext, EditorStage};
use std::{borrow::Cow, collections::HashMap};

pub const NUM_EDITOR_FIELDS: usize = 3;

#[derive(Clone)]
pub struct EditorField {
    pub stage: EditorStage,
    pub title: Cow<'static, str>,
    pub modifiable: bool,
    pub key: Cow<'static, str>,
    pub value: String,
    pub num_lines: u16,
}

impl EditorField {
    pub fn build_initial_fields() -> [EditorField; NUM_EDITOR_FIELDS] {
        [
            EditorField {
                stage: EditorStage::Id,
                title: Cow::Borrowed(" Task ID "),
                modifiable: false,
                key: Cow::Borrowed("id"),
                value: String::new(),
                num_lines: 3,
            },
            EditorField {
                stage: EditorStage::Content,
                title: Cow::Borrowed(" Content "),
                modifiable: true,
                key: Cow::Borrowed("content"),
                value: String::new(),
                num_lines: 3,
            },
            EditorField {
                stage: EditorStage::Description,
                title: Cow::Borrowed(" Description "),
                modifiable: true,
                key: Cow::Borrowed("description"),
                value: String::new(),
                num_lines: 10,
            },
        ]
    }

    pub fn get_field_index(stage: EditorStage) -> usize {
        match stage {
            EditorStage::Id => 0,
            EditorStage::Content => 1,
            EditorStage::Description => 2,
        }
    }
}

impl EditorContext {
    pub fn fields(&self) -> &[EditorField; NUM_EDITOR_FIELDS] {
        &self.fields
    }

    pub fn get_field(&self, stage: EditorStage) -> &EditorField {
        &self.fields[EditorField::get_field_index(stage)]
    }

    pub fn set_fields(&mut self, values: [&String; NUM_EDITOR_FIELDS]) {
        for (index, value) in values.into_iter().enumerate() {
            self.fields[index].value = value.clone();
        }
    }

    pub fn build_body(&self) -> HashMap<&str, &str> {
        let mut body: HashMap<&str, &str> = HashMap::new();
        for i in 1..NUM_EDITOR_FIELDS {
            body.insert(self.fields[i].key.as_ref(), self.fields[i].value.as_str());
        }

        body
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
