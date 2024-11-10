use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::Text,
    widgets::{Block, Paragraph},
    Frame,
};

use super::ModalTrait;

const PROJECT_MODAL_TITLE: &str = " Project ";

pub struct ProjectModal {}

impl ProjectModal {
    pub fn new() -> ProjectModal {
        ProjectModal {}
    }
}

impl ModalTrait for ProjectModal {
    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new(Text::from("hello")).block(
                Block::bordered()
                    .style(Style::default())
                    .title(PROJECT_MODAL_TITLE),
            ),
            area,
        );
    }

    fn get_vertical_layout_constraints(&self) -> Layout {
        Layout::vertical([Constraint::Percentage(60)])
    }

    fn get_horizontal_layout_constraints(&self) -> Layout {
        Layout::vertical([Constraint::Percentage(60)])
    }
}
