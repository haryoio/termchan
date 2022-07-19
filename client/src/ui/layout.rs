use std::sync::Arc;

use tokio::sync::Mutex;
use tui::{
    buffer::Buffer,
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::Style,
    text::Text,
    widgets::{Block, ListItem, ListState, StatefulWidget, Widget},
};

pub fn split_area(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .vertical_margin(1)
        .split(area);

    chunks
}

pub fn single_area(area: Rect) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .vertical_margin(1)
        .split(area);

    chunks[0]
}
