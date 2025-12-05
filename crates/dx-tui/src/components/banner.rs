use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::Text;
use ratatui::prelude::*;

pub struct Banner;

impl Banner {
    pub fn widget<'a>() -> Paragraph<'a> {
        Paragraph::new(Text::raw("Dx - Enhanced Development Experience"))
            .block(Block::default().borders(Borders::ALL).title("Banner"))
            .alignment(Alignment::Center)
    }
}
