use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
    Frame,
};

pub fn draw_popup<B: Backend>(f: &mut Frame<B>) -> Rect {
    let y = f.size().height / 4;
    let x = f.size().width / 4;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10)].as_ref())
        .split(Rect {
            x,
            y,
            width: x * 2,
            height: y * 2,
        });
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Reset))
        .inner(chunks[0]);
    popup_block
}
