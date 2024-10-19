use ratatui::{
    layout::Rect,
    style::Color,
    text::Text,
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};

pub struct About {}

const TITLE: &str = " About ";
const CONTENT: &str = r#"
 _________  ________  ________  ___  ___  ___     
|\___   ___\\   __  \|\   ___ \|\  \|\  \|\  \    
\|___ \  \_\ \  \|\  \ \  \_|\ \ \  \\\  \ \  \   
     \ \  \ \ \  \\\  \ \  \ \\ \ \  \\\  \ \  \  
      \ \  \ \ \  \\\  \ \  \_\\ \ \  \\\  \ \  \ 
       \ \__\ \ \_______\ \_______\ \_______\ \__\
        \|__|  \|_______|\|_______|\|_______|\|__|


# What is 'todui'?
'todui' is an interactive text-based terminal user interface for Todoist, a productivity application for organizing to-do lists and managing tasks.
'todui' aims to provide essential features of Todoist so that you can improve your workflow with a simple and easy-to-use interface.

## Creator
- Jin Hyoung Joo (hyoungjoo.j@gmail.com)

"#;

impl About {
    pub fn new() -> About {
        About {}
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new(Text::styled(CONTENT, Color::White))
                .wrap(Wrap::default())
                .block(
                    Block::default()
                        .padding(Padding::new(5, 5, 1, 1))
                        .borders(Borders::ALL)
                        .style(Color::White)
                        .title(TITLE),
                ),
            area,
        );
    }
}
