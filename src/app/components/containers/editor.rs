use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

use crate::model::task::Task;

const EDITOR_TITLE: &str = " Editor ";
const ID_TITLE: &str = " Task ID ";
const CONTENT_TITLE: &str = " Content ";
const DESCRIPTION_TITLE: &str = " Description ";

pub struct Editor {}

impl Editor {
    pub fn new() -> Editor {
        Editor {}
    }

    pub fn render(&self, props: EditorProps, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .padding(Padding::new(1, 1, 3, 3))
            .borders(Borders::ALL)
            .style(Color::Green)
            .title(EDITOR_TITLE);

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let EditorProps { task } = props;

        let task = match task {
            Some(task) => task,
            None => {
                return;
            }
        };

        let id = Paragraph::new(Text::styled(task.id(), Style::default().fg(Color::White)))
            .block(Block::bordered().style(Color::White).title(ID_TITLE));

        let content = Paragraph::new(Text::styled(
            task.content(),
            Style::default().fg(Color::White),
        ))
        .block(Block::bordered().style(Color::White).title(CONTENT_TITLE));

        let description = Paragraph::new(Text::styled(
            task.description(),
            Style::default().fg(Color::White),
        ))
        .block(
            Block::bordered()
                .style(Color::White)
                .title(DESCRIPTION_TITLE),
        );

        let panels = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Max(3), Constraint::Max(10)])
            .split(inner_area);

        frame.render_widget(id, panels[0]);
        frame.render_widget(content, panels[1]);
        frame.render_widget(description, panels[2]);
    }
}

pub struct EditorProps<'a> {
    task: Option<&'a Task>,
}

impl<'a> EditorProps<'a> {
    pub fn new(task: Option<&'a Task>) -> EditorProps<'a> {
        EditorProps { task }
    }
}
