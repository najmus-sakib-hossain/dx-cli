use ratatui::widgets::{Block, Borders, Gauge};
use ratatui::prelude::*;

pub struct Progress;

impl Progress {
    pub fn widget() -> Gauge<'static> {
        Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Progress"))
            .gauge_style(Style::default().fg(Color::Green))
            .ratio(0.42)
    }
}
