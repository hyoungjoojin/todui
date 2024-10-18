use crate::{
    app::{context::editor::EditorStage, Context},
    model::{task::Task, Model},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

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

        let EditorProps {
            stage,
            id,
            content,
            description,
        } = props;

        let id = Paragraph::new(Text::styled(id, Style::default().fg(Color::White)))
            .block(Block::bordered().style(Color::White).title(ID_TITLE));

        let content = Paragraph::new(Text::styled(content, Style::default().fg(Color::White)))
            .block(
                Block::bordered()
                    .style(if stage == EditorStage::CONTENT {
                        Color::Green
                    } else {
                        Color::White
                    })
                    .title(CONTENT_TITLE),
            );

        let description =
            Paragraph::new(Text::styled(description, Style::default().fg(Color::White))).block(
                Block::bordered()
                    .style(if stage == EditorStage::DESCRIPTION {
                        Color::Green
                    } else {
                        Color::White
                    })
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
    stage: EditorStage,
    id: String,
    content: &'a String,
    description: &'a String,
}

impl<'a> From<(&Model, &'a Context)> for EditorProps<'a> {
    fn from((_, context): (&Model, &'a Context)) -> EditorProps<'a> {
        let id = match context.selected_task() {
            Some(task) => task.id().clone(),
            None => String::new(),
        };

        EditorProps {
            stage: *context.editor_context().stage(),
            id,
            content: context.editor_context().content(),
            description: context.editor_context().description(),
        }
    }
}
