use ratatui::{
    layout::{Layout, Rect},
    Frame,
};

pub mod help;
pub mod project;

pub trait ModalTrait {
    fn render(&self, frame: &mut Frame, area: Rect);
    fn get_vertical_layout_constraints(&self) -> Layout;
    fn get_horizontal_layout_constraints(&self) -> Layout;
}
