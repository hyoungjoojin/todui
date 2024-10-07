use crate::{
    app::context::{Context, SidebarStage, Stage},
    model::Model,
};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const TITLE: &str = " [1] - Menu ";

pub struct Menu {}

impl Menu {
    pub fn new() -> Menu {
        Menu {}
    }

    pub fn render(&self, props: MenuProps, frame: &mut Frame, area: Rect) {
        let MenuProps { on } = props;

        let color = if on { Color::Green } else { Color::White };

        frame.render_widget(
            Paragraph::new(Text::styled("", Style::default().fg(Color::White))).block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(color)
                    .title(TITLE),
            ),
            area,
        );
    }
}

pub struct MenuProps {
    on: bool,
}

impl From<(&Model, &Context)> for MenuProps {
    fn from((_, context): (&Model, &Context)) -> MenuProps {
        let on =
            *context.stage() == Stage::SIDEBAR && *context.sidebar_stage() == SidebarStage::MENU;

        MenuProps { on }
    }
}
