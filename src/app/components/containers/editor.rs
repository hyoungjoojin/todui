use crate::{
    app::{
        context::editor::{EditorField, EditorStage},
        Context,
    },
    model::Model,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

const EDITOR_TITLE: &str = " Editor ";

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

        let EditorProps { stage, fields } = props;

        let panels = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                fields
                    .to_vec()
                    .iter()
                    .map(|field| Constraint::Max(field.num_lines))
                    .collect::<Vec<Constraint>>(),
            )
            .split(inner_area);

        let _ = fields
            .to_vec()
            .iter()
            .map(|field| {
                Paragraph::new(Text::styled(
                    field.value.clone(),
                    Style::default().fg(Color::White),
                ))
                .block(
                    Block::bordered()
                        .style(if field.modifiable && stage == field.stage {
                            Color::Green
                        } else {
                            Color::White
                        })
                        .title(field.title.as_ref()),
                )
            })
            .enumerate()
            .for_each(|(i, panel)| {
                frame.render_widget(panel, panels[i]);
            });
    }
}

pub struct EditorProps<'a> {
    stage: EditorStage,
    fields: &'a [EditorField; 3],
}

impl<'a> From<(&Model, &'a Context)> for EditorProps<'a> {
    fn from((_, context): (&Model, &'a Context)) -> EditorProps<'a> {
        EditorProps {
            stage: *context.editor_context().stage(),
            fields: context.editor_context().fields(),
        }
    }
}
