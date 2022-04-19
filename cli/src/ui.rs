use crate::state::{InputMode, Pane, TabItem};
use futures::executor::block_on;
use std::{io::Write, vec};

use crate::state::State;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

pub fn draw(frame: &mut Frame<CrosstermBackend<impl Write>>, state: &mut State, chunk: Rect) {
    let current_tab = state.history.last().unwrap_or(&TabItem::Bbsmenu);
    // 一番上のレイアウトを定義
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10)].as_ref())
        .split(chunk);

    match current_tab {
        TabItem::Bbsmenu => {
            let board_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(chunks[0]);
            let (left, right) = draw_bbsmenu(&mut state.clone());
            let category_state = &state.clone().category.state;
            frame.render_stateful_widget(left, board_chunks[0], &mut category_state.to_owned());
            let board_state = &state.clone().boards.state;
            frame.render_stateful_widget(right, board_chunks[1], &mut board_state.to_owned());
        }
        TabItem::Board => {
            let board_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                .split(chunks[0]);

            let (left, right) = draw_board(&mut state.clone());
            let thread_list_state = &state.clone().threads.state;
            frame.render_stateful_widget(left, board_chunk[0], &mut thread_list_state.to_owned());
            let reply_list_state = &state.clone().thread.state;
            frame.render_stateful_widget(right, board_chunk[1], &mut reply_list_state.to_owned());
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

fn draw_bbsmenu<'a>(app: &mut State) -> (List<'a>, List<'a>) {
    // カテゴリリスト用のブロックを作成
    let category_list_block = Block::default()
        .borders(Borders::all())
        .style(Style::default().fg(if app.focus_pane.get() == Pane::Left {
            Color::White
        } else {
            Color::Black
        }))
        .title("BoardCategory")
        .border_type(BorderType::Plain);

    let category_items: Vec<ListItem> = app
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

    // 板リスト用のブロックを作成
    let board_block = Block::default()
        .borders(Borders::all())
        .style(Style::default().fg(if app.focus_pane.get() == Pane::Right {
            Color::White
        } else {
            Color::Black
        }))
        .title("BoardList")
        .border_type(BorderType::Plain);

    let board_items: Vec<ListItem> = app
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

    (category_list, board_list)
}

fn draw_board<'a>(app: &mut State) -> (List<'a>, List<'a>) {
    // スレッドリスト用のブロックを作成
    let thread_list_block = Block::default()
        .borders(Borders::all())
        .style(Style::default().fg(if app.focus_pane.get() == Pane::Left {
            Color::White
        } else {
            Color::Black
        }))
        .title(app.current_board().title.clone())
        .border_type(BorderType::Plain);

    // stateのスレッド一覧をListItemへ変換
    let thread_items: Vec<ListItem> = app
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

    // リプライ用のブロックを作成
    let reply_list_block = Block::default()
        .borders(Borders::all())
        .style(Style::default().fg(if app.focus_pane.get() == Pane::Right {
            Color::White
        } else {
            Color::Black
        }))
        .title("Thread")
        .border_type(BorderType::Plain);

    // stateからリプライ一覧を取得、ListItemへ変換
    // TODO: messageからURLなどを抜き出しリンク化
    let reply_items: Vec<ListItem> = app
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

    // TODO: Listを作成
    let reply_list = List::new(reply_items)
        .block(reply_list_block)
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    (thread_list, reply_list)
}
