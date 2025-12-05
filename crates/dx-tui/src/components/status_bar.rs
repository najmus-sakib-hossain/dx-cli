use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::Text;

pub struct StatusBar;

impl StatusBar {
    pub fn widget<'a>() -> Paragraph<'a> {
        Paragraph::new(Text::raw("Ready"))
            .block(Block::default().borders(Borders::ALL).title("Status"))
    }
}
