use crate::app::{
    components::containers::modal::{help::HelpModal, project::ProjectModal, ModalTrait},
    context::ModalStage,
    Context,
};
use ratatui::{
    layout::{Flex, Rect},
    widgets::Clear,
    Frame,
};

pub struct Modal {
    modal: Option<Box<dyn ModalTrait>>,
    current_stage: ModalStage,
}

impl Modal {
    pub fn new() -> Modal {
        Modal {
            modal: None,
            current_stage: ModalStage::Off,
        }
    }

    pub fn render(&mut self, context: &Context, frame: &mut Frame, area: Rect) {
        if self.current_stage != context.modal_stage() {
            self.current_stage = context.modal_stage();

            match self.current_stage {
                ModalStage::Help => {
                    self.modal.replace(Box::new(HelpModal::new()));
                }
                ModalStage::Project => {
                    self.modal.replace(Box::new(ProjectModal::new()));
                }
                ModalStage::Off => {
                    self.modal.take();
                    return;
                }
            };
        }

        let modal = if let Some(modal) = &self.modal {
            modal
        } else {
            return;
        };

        let [area] = modal
            .get_vertical_layout_constraints()
            .flex(Flex::Center)
            .areas(area);

        let [area] = modal
            .get_horizontal_layout_constraints()
            .flex(Flex::Center)
            .areas(area);

        frame.render_widget(Clear, area);

        modal.render(frame, area);
    }
}
