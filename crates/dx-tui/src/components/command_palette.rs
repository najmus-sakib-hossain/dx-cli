use ratatui::widgets::{Block, Borders, List, ListItem};

pub struct CommandPalette;

impl CommandPalette {
    pub fn widget<'a>() -> List<'a> {
        let items = vec![ListItem::new("Type to search commands...")];
        List::new(items).block(Block::default().borders(Borders::ALL).title("Command Palette"))
    }
}
