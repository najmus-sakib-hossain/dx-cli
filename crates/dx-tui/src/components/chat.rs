use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::Text;

pub struct Chat;

impl Chat {
    pub fn widget<'a>() -> Paragraph<'a> {
        Paragraph::new(Text::raw("Chat area (placeholder)"))
            .block(Block::default().borders(Borders::ALL).title("Chat"))
    }
}
