use crate::state::{InputMode, Pane, TabItem};
use crossterm::{cursor, execute, terminal};
use futures::executor::block_on;
use std::{
    io::{stdout, Write},
    vec,
};

use crate::state::State;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};
pub fn draw_board_view<'a, B: Backend>(frame: &mut Frame<B>, state: &mut State, chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunk);
    draw_thread_list(frame, state, chunks[0]);
    draw_thread(frame, state, chunks[1])
}

pub fn draw_thread_list<'a, B: Backend>(frame: &mut Frame<B>, state: &mut State, chunk: Rect) {
    // スレッドリスト用のブロックを作成
    let thread_list_block = Block::default()
        .borders(Borders::all())
        .style(
            Style::default().fg(if state.focus_pane.get() == Pane::Left {
                Color::White
            } else {
                Color::Black
            }),
        )
        .title(state.current_board().title.clone())
        .border_type(BorderType::Plain);

    // stateのスレッド一覧をListItemへ変換
    let thread_items: Vec<ListItem> = state
        .threads
        .items
        .iter()
        .map(|thread| {
            ListItem::new(Span::styled(
                thread.title.clone(),
                Style::default().fg(Color::White),
            ))
        })
        .collect();

    // Listを作成
    let thread_list = List::new(thread_items)
        .block(thread_list_block)
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    let thread_list_state = &state.clone().threads.state;
    frame.render_stateful_widget(thread_list, chunk, &mut thread_list_state.to_owned());
}

pub fn draw_thread<'a, B: Backend>(frame: &mut Frame<B>, state: &mut State, chunk: Rect) {
    // リプライ用のブロックを作成
    let reply_list_block = Block::default()
        .borders(Borders::all())
        .style(
            Style::default().fg(if state.focus_pane.get() == Pane::Right {
                Color::White
            } else {
                Color::Black
            }),
        )
        .title("Thread")
        .border_type(BorderType::Plain);

    // stateからリプライ一覧を取得、ListItemへ変換
    let reply_items: Vec<ListItem> = state
        .thread
        .to_vec()
        .iter()
        .map(|reply| {
            let mut spans = vec![
                Spans::from(vec![
                    Span::styled(reply.reply_id.clone(), Style::default().fg(Color::White)),
                    Span::styled(
                        reply.name.clone(),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Spans::from(vec![]),
            ];

            for message in reply.message.clone().split("<br>") {
                spans.push(Spans::from(vec![Span::styled(
                    message.to_string(),
                    Style::default().fg(Color::White),
                )]));
            }

            let text = Text::from(spans);

            ListItem::new(text)
        })
        .collect();

    let reply_list = List::new(reply_items)
        .block(reply_list_block)
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    let reply_list_state = &state.clone().thread.state;
    frame.render_stateful_widget(reply_list, chunk, &mut reply_list_state.to_owned());
}
