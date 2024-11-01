use crate::app::{components::containers::modal::help::HelpModal, context::ModalStage, Context};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Clear,
    Frame,
};

pub struct Modal {}

impl Modal {
    pub fn new() -> Modal {
        Modal {}
    }

    pub fn render(&self, context: &Context, frame: &mut Frame, area: Rect) {
        if context.modal_stage() == ModalStage::OFF {
            return;
        }

        let [area] = Layout::vertical([Constraint::Percentage(60)])
            .flex(Flex::Center)
            .areas(area);

        let [area] = Layout::horizontal([Constraint::Percentage(60)])
            .flex(Flex::Center)
            .areas(area);

        frame.render_widget(Clear, area);

        let modal = match context.modal_stage() {
            ModalStage::HELP => HelpModal::new(),
            _ => {
                return;
            }
        };

        modal.render(frame, area);
    }
}
