mod bbsmenu_view;
mod board_view;
mod settings_view;

use crate::state::{InputMode, TabItem};
use futures::executor::block_on;
use std::io::Write;

use crate::state::State;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use self::{bbsmenu_view::draw_bbsmenu, board_view::draw_board_view};

pub fn draw(frame: &mut Frame<CrosstermBackend<impl Write>>, state: &mut State, chunk: Rect) {
    let current_tab = state.history.last().unwrap_or(&TabItem::Bbsmenu);
    // 一番上のレイアウトを定義
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10)].as_ref())
        .split(chunk);

    match current_tab {
        TabItem::Bbsmenu => {
            draw_bbsmenu(frame, state, chunks[0]);
        }
        TabItem::Board => {
            draw_board_view(frame, state, chunks[0]);
        }
        TabItem::Settings => todo!(),
    }

    block_on(state.reply_form.render(frame));
    match state.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => {
            let chunk = state.reply_form.current_chunk();
            let chunk = match chunk {
                Some(chunk) => chunk,
                None => todo!(),
            };
            let width = block_on(state.reply_form.width()) + 1;
            let height = block_on(state.reply_form.height()) + 1;
            frame.set_cursor(chunk.x + width as u16, chunk.y + height as u16);
        }
        InputMode::Input => {}
    };
}
