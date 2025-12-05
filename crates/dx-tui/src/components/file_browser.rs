use ratatui::widgets::{Block, Borders, List, ListItem};

pub struct FileBrowser;

impl FileBrowser {
    pub fn widget<'a>() -> List<'a> {
        let items = vec![ListItem::new("./ (stub)")];
        List::new(items).block(Block::default().borders(Borders::ALL).title("Files"))
    }
}
