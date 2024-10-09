use crate::{
    app::context::{Context, MenuStage, SidebarStage, Stage},
    model::Model,
};
use ratatui::{
    layout::Rect,
    style::Color,
    text::{Line, Span, Text},
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
        let MenuProps { on, menu_stage } = props;

        let text = vec![
            Line::from(Span::styled(
                "- Today ",
                match menu_stage {
                    MenuStage::TODAY => Color::Green,
                    MenuStage::UPCOMING => Color::White,
                },
            )),
            Line::from(Span::styled(
                "- Upcoming ",
                match menu_stage {
                    MenuStage::TODAY => Color::White,
                    MenuStage::UPCOMING => Color::Green,
                },
            )),
        ];

        frame.render_widget(
            Paragraph::new(Text::from(text)).block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(if on { Color::Green } else { Color::White })
                    .title(TITLE),
            ),
            area,
        );
    }
}

pub struct MenuProps {
    on: bool,
    menu_stage: MenuStage,
}

impl From<(&Model, &Context)> for MenuProps {
    fn from((_, context): (&Model, &Context)) -> MenuProps {
        let on =
            *context.stage() == Stage::SIDEBAR && *context.sidebar_stage() == SidebarStage::MENU;

        MenuProps {
            on,
            menu_stage: context.menu_stage(),
        }
    }
}
