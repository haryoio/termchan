use crate::state::Pane;

use crate::state::State;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

pub fn draw_bbsmenu<'a, B: Backend>(frame: &mut Frame<B>, state: &mut State, chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunk);
    draw_category_list(frame, state, chunks[0]);
    draw_board_list(frame, state, chunks[1]);
}

pub fn draw_category_list<'a, B: Backend>(frame: &mut Frame<B>, state: &mut State, chunk: Rect) {
    // カテゴリリスト用のブロックを作成
    let category_list_block = Block::default()
        .borders(Borders::all())
        .style(
            Style::default().fg(if state.focus_pane.get() == Pane::Left {
                Color::White
            } else {
                Color::Black
            }),
        )
        .title("BoardCategory")
        .border_type(BorderType::Plain);

    let category_items: Vec<ListItem> = state
        .category
        .items
        .iter()
        .map(|category| {
            ListItem::new(Span::styled(
                category.category.to_string(),
                Style::default().fg(Color::White),
            ))
        })
        .collect();

    let category_list = List::new(category_items)
        .block(category_list_block)
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    let category_state = &state.clone().category.state;
    frame.render_stateful_widget(category_list, chunk, &mut category_state.to_owned());
}

pub fn draw_board_list<'a, B: Backend>(frame: &mut Frame<B>, state: &mut State, chunk: Rect) {
    // 板リスト用のブロックを作成
    let board_block = Block::default()
        .borders(Borders::all())
        .style(
            Style::default().fg(if state.focus_pane.get() == Pane::Right {
                Color::White
            } else {
                Color::Black
            }),
        )
        .title("BoardList")
        .border_type(BorderType::Plain);

    let board_items: Vec<ListItem> = state
        .boards
        .to_vec()
        .iter()
        .map(|board| {
            ListItem::new(Span::styled(
                board.title.clone(),
                Style::default().fg(Color::White),
            ))
        })
        .collect();

    let board_list = List::new(board_items).block(board_block).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let board_state = &state.clone().boards.state;
    frame.render_stateful_widget(board_list, chunk, &mut board_state.to_owned());
}
