use crate::{
    app::context::{Context, Stage},
    model::Model,
};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Tasks {}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks {}
    }

    pub fn render(&self, props: TasksProps, frame: &mut Frame, area: Rect) {
        let TasksProps { on } = props;

        let color = if on { Color::Green } else { Color::White };

        frame.render_widget(
            Paragraph::new(Text::styled("", Style::default().fg(Color::White))).block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(color)
                    .title(" Tasks "),
            ),
            area,
        );
    }
}

pub struct TasksProps {
    on: bool,
}

impl From<(&Model, &Context)> for TasksProps {
    fn from((_, context): (&Model, &Context)) -> TasksProps {
        let on = *context.stage() != Stage::SIDEBAR;

        TasksProps { on }
    }
}
