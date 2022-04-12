use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell},
    error::Error,
    io,
    ops::DerefMut,
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
    thread::{self, Thread},
    time::Duration,
};

use futures::executor::block_on;

use anyhow::Context;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
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
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
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
/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
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
    // リサイズモードで左右ペインの比率を変更できるモードへ移行
    //Resize,
}

#[derive(Debug, Clone)]
struct State {
    pub category_list_state: ListState,
    pub board_list_state: ListState,
    pub thread_list_state: ListState,
    pub reply_list_state: ListState,
    pub current_history: TabItem,
    pub history: Vec<TabItem>,
    pub category_list: Vec<BbsCategories>,
    pub board_list: Arc<Mutex<RefCell<Vec<BoardUrl>>>>,
    pub thread_list: Vec<TCThread>,
    pub reply_list: Vec<Reply>,
    pub board_url: String,
    pub focus_left_or_right: Cell<Pane>,
}

impl State {
    pub fn new() -> State {
        let mut category_list_state = ListState::default();
        let mut board_list_state = ListState::default();
        let mut thread_list_state = ListState::default();
        let mut reply_list_state = ListState::default();

        category_list_state.select(Some(0));
        board_list_state.select(Some(0));
        thread_list_state.select(Some(0));
        reply_list_state.select(Some(0));

        State {
            category_list_state,
            board_list_state,
            thread_list_state,
            reply_list_state,
            current_history: TabItem::Bbsmenu,
            history: Vec::new(),
            category_list: Vec::new(),
            board_list: Arc::new(Mutex::new(RefCell::new(Vec::new()))),
            thread_list: Vec::new(),
            reply_list: Vec::new(),
            board_url: String::new(),
            focus_left_or_right: Cell::new(Pane::Left),
        }
    }
    pub fn current_category(&self) -> &BbsCategories {
        &self.category_list[self.category_list_state.selected().unwrap()]
    }

    pub fn current_board(&self) -> &BoardUrl {
        let selected_board = self.board_list_state.selected().unwrap();
        &self.current_category().list[selected_board]
    }

    pub fn set_board_list(&mut self, list: Vec<BoardUrl>) {
        self.board_list.lock().unwrap().replace(list);
    }

    pub fn add_history(&mut self, item: TabItem) {
        self.current_history = item.clone();
        self.history.push(item);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
    let category_list = block_on(BbsMenu::new(bbsmenu_url.to_string()).load()).unwrap();
    let board_list = Arc::new(Mutex::new(RefCell::new(category_list[0].list.clone())));

    // ヒストリーの初期値としてBBSMENUを設定
    let history = vec![TabItem::Bbsmenu];

    let mut state = State::new();
    state.category_list = category_list;
    state.board_list = board_list;
    state.history = history;

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

    loop {
        terminal.draw(|f| {
            let size = f.size();
            // 一番上のレイアウトを定義
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Min(10)].as_ref())
                .split(size);

            let current_tab = state.history.last().unwrap();

            match current_tab {
                TabItem::Bbsmenu => {
                    let board_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[0]);
                    let (left, right) = render_bbsmenu(&mut state.clone());
                    let category_state = &state.clone().category_list_state;
                    f.render_stateful_widget(left, board_chunks[0], &mut category_state.to_owned());
                    let board_state = &state.clone().board_list_state;
                    f.render_stateful_widget(right, board_chunks[1], &mut board_state.to_owned());
                }
                TabItem::Board => {
                    let board_chunk = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(40), Constraint::Percentage(60)].as_ref(),
                        )
                        .split(chunks[0]);
                    let (left, right) = render_board(&mut state.clone());
                    let thread_list_state = &state.clone().thread_list_state;
                    f.render_stateful_widget(
                        left,
                        board_chunk[0],
                        &mut thread_list_state.to_owned(),
                    );
                    let reply_list_state = &state.clone().reply_list_state;
                    f.render_stateful_widget(
                        right,
                        board_chunk[1],
                        &mut reply_list_state.to_owned(),
                    );
                }
                TabItem::Thread => todo!(),
                TabItem::Settings => todo!(),
            }
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Up => {
                    match &state.current_history {
                        TabItem::Bbsmenu => match &state.focus_left_or_right.get() {
                            // Category
                            Pane::Left => {
                                let selected = state.category_list_state.selected();
                                if selected.is_some() {
                                    state.category_list_state.select(selected.and_then(|i| {
                                        if i <= 0 {
                                            Some(state.category_list.len() - 1)
                                        } else {
                                            Some(i - 1)
                                        }
                                    }));
                                }
                                let selected_category = state.current_category();
                                let mut state = state.clone();
                                state.set_board_list(selected_category.list.clone());
                            }
                            // BoardList
                            Pane::Right => {
                                let selected = state.board_list_state.selected();
                                if selected.is_some() {
                                    state.board_list_state.select(selected.and_then(|i| {
                                        let len = (*state.board_list.lock().unwrap().borrow_mut())
                                            .get_mut()
                                            .len();
                                        if i <= 0 {
                                            Some(len - 1)
                                        } else {
                                            Some(i - 1)
                                        }
                                    }));
                                }
                            }
                        },
                        TabItem::Board => todo!(),
                        TabItem::Thread => todo!(),
                        TabItem::Settings => todo!(),
                    };
                }
                KeyCode::Down => {
                    match &state.current_history {
                        TabItem::Bbsmenu => match &state.focus_left_or_right.get() {
                            // Category
                            Pane::Left => {
                                let selected = state.category_list_state.selected();

                                if selected.is_some() {
                                    state.category_list_state.select(selected.and_then(|i| {
                                        if i >= state.category_list.len() - 1 {
                                            Some(0)
                                        } else {
                                            Some(i + 1)
                                        }
                                    }));
                                }
                                let selected_category = state.current_category();
                                let mut state = state.clone();
                                state.set_board_list(selected_category.list.clone());
                            }
                            // BoardList
                            Pane::Right => {
                                let selected = state.board_list_state.selected();
                                if selected.is_some() {
                                    state.board_list_state.select(selected.and_then(|i| {
                                        let len = (*state.board_list.lock().unwrap().borrow_mut())
                                            .get_mut()
                                            .len();
                                        if i >= len - 1 {
                                            Some(0)
                                        } else {
                                            Some(i + 1)
                                        }
                                    }));
                                }
                            }
                        },
                        TabItem::Board => todo!(),
                        TabItem::Thread => todo!(),
                        TabItem::Settings => todo!(),
                    };
                }

                KeyCode::Enter => {
                    if state.focus_left_or_right.get() == Pane::Left {
                        match &state.current_history {
                            // 左ペインでEnterを押すと、左右ペインへ移動する。
                            TabItem::Bbsmenu => state.focus_left_or_right.set(Pane::Right),
                            TabItem::Board => todo!(),
                            TabItem::Thread => todo!(),
                            TabItem::Settings => todo!(),
                        }
                    } else {
                        // 右ペインでEnterを押すと、次のタブへ移動する
                        match &state.current_history {
                            // 板を選択,スレッド一覧画面へ移行
                            TabItem::Bbsmenu => {
                                // 選択した板URLを取得
                                state.board_url = state.current_board().url.clone();
                                // TODO: 選択した板からスレッド一覧を取得する
                                // TODO: スレッド一覧をStateへコピー
                                // ヒストリに板タブを追加
                                state.add_history(TabItem::Board);
                            }
                            TabItem::Board => {
                                //
                                todo!()
                            }
                            TabItem::Thread => todo!(),
                            TabItem::Settings => todo!(),
                        };
                    }
                }
                KeyCode::Left => state.focus_left_or_right.set(Pane::Left),
                KeyCode::Right => state.focus_left_or_right.set(Pane::Right),
                _ => {}
            },
            Event::Tick => {}
            Event::Init => {}
        }
    }

    Ok(())
}

fn render_bbsmenu<'a>(state: &mut State) -> (List<'a>, List<'a>) {
    // カテゴリリスト用のブロックを作成
    let category_list_block = Block::default()
        .borders(Borders::all())
        .style(
            Style::default().fg(if state.focus_left_or_right.get() == Pane::Left {
                Color::White
            } else {
                Color::Black
            }),
        )
        .title("BoardCategory")
        .border_type(BorderType::Plain);

    let category_items: Vec<ListItem> = state
        .category_list
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
        .style(
            Style::default().fg(if state.focus_left_or_right.get() == Pane::Right {
                Color::White
            } else {
                Color::Black
            }),
        )
        .title("BoardList")
        .border_type(BorderType::Plain);

    let board_items: Vec<ListItem> = state
        .board_list
        .lock()
        .unwrap()
        .borrow_mut()
        .get_mut()
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

fn render_board<'a>(state: &mut State) -> (List<'a>, List<'a>) {
    // TODO: スレッドリスト用のブロックを作成
    let thread_list_block = unimplemented!();

    // TODO: stateのスレッド一覧をListItemへ変換
    let thread_items: Vec<ListItem> = unimplemented!();

    // TODO: Listを作成
    let thread_list = unimplemented!();

    // TODO: リプライ用のブロックを作成
    let reply_list_block = unimplemented!();

    // TODO: stateからリプライ一覧を取得、ListItemへ変換
    //  リプライがない場合はダミーデータ(空の構造体)からListItemを作成
    let reply_items: Vec<ListItem> = unimplemented!();

    // TODO: Listを作成
    let reply_list = unimplemented!();

    // (thread_list, reply_list)
}
