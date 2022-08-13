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

#[allow(dead_code)]
pub fn split_vertical(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .horizontal_margin(1)
        .split(area);

    chunks
}

#[allow(dead_code)]
pub fn thread_form_area(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(3),
            ]
            .as_ref(),
        )
        .split(area);

    chunks
}
