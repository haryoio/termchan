use tui::layout::{Constraint, Direction, Layout, Rect};

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

pub fn split_area_vertical(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(10)].as_ref())
        .horizontal_margin(1)
        .split(area);

    chunks
}

pub fn popup_area(area: Rect) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .vertical_margin(1)
        .split(area);

    chunks[0]
}
