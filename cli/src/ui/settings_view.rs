use crate::state::State;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn draw_settings_view<'a, B: Backend>(frame: &mut Frame<B>, state: &mut State, chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunk);
    draw_settings_list(frame, state, chunks[0]);
    draw_settings_detail(frame, state, chunks[1]);
}

pub fn draw_settings_list<'a, B: Backend>(frame: &mut Frame<B>, state: &mut State, chunk: Rect) {}

pub fn draw_settings_detail<'a, B: Backend>(frame: &mut Frame<B>, state: &mut State, chunk: Rect) {}
