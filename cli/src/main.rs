extern crate cli;

use cli::widgets::{atomic_stateful_list::AtomicStatefulList, stateful_list::StatefulList};
use std::{cell::Cell, io, sync::mpsc, thread, time::Duration, vec};

use anyhow::Context;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use futures::executor::block_on;
use termchan::{
    configs::config::Config,
    controller::{
        board::Board,
        menu::{BbsCategories, BbsMenu, BoardUrl},
        reply::Reply,
        thread::Thread as TCThread,
    },
};
use tokio::time::Instant;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph},
    Terminal,
};

enum Event<I> {
    Input(I),
    Tick,
    Init,
}

enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, Clone)]
enum TabItem {
    Bbsmenu,
    Board,
    Thread,
    Settings,
}

// 左右ペインへの移動
#[derive(Debug, Clone, Copy, PartialEq)]
enum Pane {
    Left,
    Right,
    //Resize,
}

#[derive(Debug, Clone)]
struct App {
    pub category: StatefulList<BbsCategories>,
    pub boards: AtomicStatefulList<BoardUrl>,
    pub threads: StatefulList<TCThread>,
    pub thread: AtomicStatefulList<Reply>,
    pub current_history: TabItem,
    pub history: Vec<TabItem>,
    pub board_url: String,
    pub focus_pane: Cell<Pane>,
}

impl App {
    pub fn new() -> Self {
        App {
            category: StatefulList::with_items(vec![]),
            boards: AtomicStatefulList::with_items(vec![]),
            threads: StatefulList::with_items(vec![]),
            thread: AtomicStatefulList::with_items(vec![]),
            current_history: TabItem::Bbsmenu,
            history: Vec::new(),
            board_url: String::new(),
            focus_pane: Cell::new(Pane::Left),
        }
    }
    pub fn current_category(&self) -> &BbsCategories {
        &self.category.items[self.category.state.selected().unwrap_or(0)]
    }

    pub fn current_board(&self) -> &BoardUrl {
        let selected_board = self.boards.state.selected().unwrap_or(0);
        &self.current_category().list[selected_board]
    }

    pub fn current_thread(&self) -> &TCThread {
        &self.threads.items[self.threads.state.selected().unwrap_or(0)]
    }

    pub fn current_reply(&self) -> &Reply {
        let selected_reply = self.threads.state.selected().unwrap();
        &self.current_thread().list[selected_reply]
    }

    pub fn add_history(&mut self, item: TabItem) {
        self.current_history = item.clone();
        self.history.push(item);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    enable_raw_mode().context("Failed to enable raw mode")?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // 設定を読み込み
    let config = Config::load();
    let bbsmenu_url = match config.unwrap().bbsmenu.url.first() {
        Some(url) => url.to_owned(),
        None => panic!("BBSMENU URLを設定してください。"),
    };

    // BBSMENUを取得
    // BBSMENU ⊃ "BoardCategory" ⊃ BoardURL
    let mut app = App::new();

    app.category.items = block_on(BbsMenu::new(bbsmenu_url.to_string()).load()).unwrap();
    app.boards.set_items(app.category.items[0].list.clone());
    app.threads.items = vec![TCThread::default()];
    app.thread.set_items(vec![Reply::default()]);
    app.history = vec![TabItem::Bbsmenu];

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_millis(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("read works") {
                    tx.send(Event::Input(key)).expect("send works");
                }
            }
            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let show_popup = false;

    loop {
        let current_tab = app.history.last().unwrap();
        terminal.draw(|f| {
            let size = f.size();
            // 一番上のレイアウトを定義
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(10)].as_ref())
                .split(size);

            match current_tab {
                TabItem::Bbsmenu => {
                    let board_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[0]);
                    let (left, right) = render_bbsmenu(&mut app.clone());

                    let category_state = &app.clone().category.state;
                    f.render_stateful_widget(left, board_chunks[0], &mut category_state.to_owned());

                    let board_state = &app.clone().boards.state;
                    f.render_stateful_widget(right, board_chunks[1], &mut board_state.to_owned());
                }
                TabItem::Board => {
                    let board_chunk = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(40), Constraint::Percentage(60)].as_ref(),
                        )
                        .split(chunks[0]);

                    let (left, right) = render_board(&mut app.clone());

                    let thread_list_state = &app.clone().threads.state;
                    f.render_stateful_widget(
                        left,
                        board_chunk[0],
                        &mut thread_list_state.to_owned(),
                    );
                    let reply_list_state = &app.clone().thread.state;
                    f.render_stateful_widget(
                        right,
                        board_chunk[1],
                        &mut reply_list_state.to_owned(),
                    );
                }
                TabItem::Thread => todo!(),
                TabItem::Settings => todo!(),
            }

            if show_popup {
                let block = Block::default().title("POPUP").borders(Borders::ALL);
                let para = Paragraph::new(Text::from(vec![Spans::from(vec![Span::styled(
                    "Hello, world!",
                    Style::default().fg(Color::Red),
                )])]))
                .block(block);
                let area = render_popup(69, 20, size);
                f.render_widget(Clear, area);
                f.render_widget(para, area);
            }
        })?;

        match rx.recv()? {
            Event::Input(event) => {
                match event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    KeyCode::Up => {
                        match &current_tab {
                            TabItem::Bbsmenu => {
                                match &app.focus_pane.get() {
                                    // Category
                                    Pane::Left => {
                                        app.category.previous();
                                        let selected_category = app.current_category();
                                        let mut app = app.clone();
                                        app.boards.set_items(selected_category.list.clone());
                                    } // Down -> Bbsmenu -> Pane::Left
                                    // BoardList
                                    Pane::Right => {
                                        app.boards.previous();
                                    }
                                }
                            }
                            TabItem::Board => {
                                match &app.focus_pane.get() {
                                    // ThreadList
                                    Pane::Left => {
                                        app.threads.previous();
                                    } // Down -> ThreadList -> Pane::Left
                                    // Thread
                                    Pane::Right => {
                                        let selected = app.thread.state.selected();
                                        if selected.is_some() {
                                            app.thread.state.select(selected.and_then(|i| {
                                                if i <= 0 {
                                                    Some(0)
                                                } else {
                                                    Some(i - 1)
                                                }
                                            }));
                                        }
                                    } // Down -> Thread -> Pane::Right
                                }
                            }
                            TabItem::Thread => todo!(),
                            TabItem::Settings => todo!(),
                        };
                    }
                    KeyCode::Down => {
                        match &current_tab {
                            TabItem::Bbsmenu => {
                                match &app.focus_pane.get() {
                                    // Category
                                    Pane::Left => {
                                        app.category.next();
                                        let selected_category = app.current_category();
                                        let mut app = app.clone();
                                        app.boards.set_items(selected_category.list.clone());
                                    } // Down -> Bbsmenu -> Pane::Left
                                    // BoardList
                                    Pane::Right => {
                                        app.boards.next();
                                    } // Down -> Bbsmenu -> Pane::Left
                                }
                            }
                            TabItem::Board => {
                                match &app.focus_pane.get() {
                                    // ThreadList
                                    Pane::Left => {
                                        app.threads.next();
                                    } // Down -> ThreadList -> Pane::Left
                                    // Thread
                                    Pane::Right => {
                                        app.thread.next();
                                    } // Down -> Thread -> Pane::Left
                                }
                            }
                            TabItem::Thread => todo!(),
                            TabItem::Settings => todo!(),
                        };
                    }

                    KeyCode::Enter => {
                        if app.focus_pane.get() == Pane::Left {
                            match &current_tab {
                                // 左ペインでEnterを押すと、右ペインへ移動する。
                                TabItem::Bbsmenu => app.focus_pane.set(Pane::Right),
                                TabItem::Board => {
                                    let mut thread = app.current_thread().clone();
                                    let reply_list = block_on(thread.load())
                                        .context("failed to load reply list")?;

                                    app.focus_pane.set(Pane::Right);
                                    app.thread.state.select(Some(0));
                                    app.thread.set_items(reply_list);
                                }
                                TabItem::Thread => todo!(),
                                TabItem::Settings => todo!(),
                            }
                            app.focus_pane.set(Pane::Right);
                        } else {
                            // 右ペインでEnterを押すと、次のタブへ移動する
                            match &current_tab {
                                // 板を選択,スレッド一覧画面へ移行
                                TabItem::Bbsmenu => {
                                    // 選択した板URLを取得
                                    app.board_url = app.current_board().url.clone();
                                    let new_threads =
                                        block_on(Board::new(app.clone().board_url).load()).unwrap();
                                    app.threads.items = new_threads;
                                    app.focus_pane.set(Pane::Left);
                                    app.add_history(TabItem::Board);
                                }
                                TabItem::Board => {}
                                TabItem::Thread => todo!(),
                                TabItem::Settings => todo!(),
                            };
                        }
                    }
                    // resizemode
                    // ペインの比率を変更する
                    KeyCode::Char('R') => {}
                    KeyCode::Left => match app.focus_pane.get() {
                        Pane::Left => match current_tab {
                            TabItem::Bbsmenu => {
                                app.focus_pane.set(Pane::Right);
                            }
                            TabItem::Board => {
                                app.history.pop();
                                app.focus_pane.set(Pane::Right);
                            }
                            TabItem::Thread => todo!(),
                            TabItem::Settings => todo!(),
                        },
                        Pane::Right => {
                            app.focus_pane.set(Pane::Left);
                        }
                    },
                    KeyCode::Right => match app.focus_pane.get() {
                        Pane::Left => app.focus_pane.set(Pane::Left),
                        Pane::Right => {
                            app.focus_pane.set(Pane::Right);
                        }
                    },
                    _ => {}
                }
            }
            Event::Tick => {}
            Event::Init => {}
        }
    }

    Ok(())
}

fn render_bbsmenu<'a>(app: &mut App) -> (List<'a>, List<'a>) {
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

fn render_board<'a>(app: &mut App) -> (List<'a>, List<'a>) {
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

fn render_popup(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
