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

const TITLE: &str = " [0] - About ";

pub struct About {}

impl About {
    pub fn new() -> About {
        About {}
    }

    pub fn render(&self, props: AboutProps, frame: &mut Frame, area: Rect) {
        let AboutProps { on } = props;

        let color = if on { Color::Green } else { Color::White };

        frame.render_widget(
            Paragraph::new(Text::styled(" todui", Style::default().fg(Color::White))).block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(color)
                    .title(TITLE),
            ),
            area,
        );
    }
}

pub struct AboutProps {
    on: bool,
}

impl From<(&Model, &Context)> for AboutProps {
    fn from((_, context): (&Model, &Context)) -> AboutProps {
        let on =
            context.stage() == Stage::SIDEBAR && context.sidebar_stage() == SidebarStage::ABOUT;

        AboutProps { on }
    }
}
