use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell},
    error::Error,
    io,
    ops::DerefMut,
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
    thread,
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
        menu::{BbsCategories, BbsMenu, BbsUrl},
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum LR {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct State {
    pub category_list_state: ListState,
    pub board_list_state: ListState,
    pub thread_list_state: ListState,
    pub current_tab_item: TabItem,
    pub history: Vec<TabItem>,
    pub category_list: Vec<BbsCategories>,
    pub board_list: Arc<Mutex<RefCell<Vec<BbsUrl>>>>,
    pub board_url: String,
    pub thread_list: Vec<TCThread>,
    pub reply_list: Vec<Reply>,
    pub focus_left_or_right: Cell<LR>,
}

impl Default for State {
    fn default() -> State {
        let bbsmenu_url = Config::load().unwrap().bbsmenu.url.clone();
        let url = match bbsmenu_url.first() {
            Some(url) => url.to_owned(),
            None => String::from("http://menu.2ch.sc/bbsmenu.html"),
        };
        let mut category_list_state = ListState::default();
        let mut board_list_state = ListState::default();
        let mut thread_list_state = ListState::default();
        category_list_state.select(Some(0));
        board_list_state.select(Some(0));
        thread_list_state.select(Some(0));
        let bbsmenu = BbsMenu::new(url.to_string());
        let category_list = block_on(bbsmenu.load()).unwrap();
        let board_first = category_list[0].list.clone();
        let board_list = Arc::new(Mutex::new(RefCell::new(board_first)));
        let history = vec![TabItem::Bbsmenu];
        State {
            category_list_state,
            board_list_state,
            thread_list_state,
            current_tab_item: TabItem::Bbsmenu,
            history,
            category_list,
            board_list,
            board_url: String::new(),
            thread_list: Vec::new(),
            reply_list: Vec::new(),
            focus_left_or_right: Cell::new(LR::Left),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode().context("Failed to enable raw mode")?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    let mut state = State::default();

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        tx.send(Event::Init);
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
                    let (left, right) = render_board(&mut state.clone());
                    let category_state = &state.clone().category_list_state;
                    f.render_stateful_widget(left, board_chunks[0], &mut category_state.to_owned());
                    let board_state = &state.clone().board_list_state;
                    f.render_stateful_widget(right, board_chunks[1], &mut board_state.to_owned());
                }
                TabItem::Board => todo!(),
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
                KeyCode::Down => {
                    match &state.current_tab_item {
                        TabItem::Bbsmenu => match &state.focus_left_or_right.get() {
                            // Category
                            LR::Left => {
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
                                let selected = state.category_list_state.selected();
                                let selected_category =
                                    state.category_list.get(selected.unwrap()).unwrap();
                                let board_list = state.board_list.clone();
                                board_list
                                    .lock()
                                    .unwrap()
                                    .replace(selected_category.list.clone());
                            }
                            // BoardList
                            LR::Right => {
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
                KeyCode::Up => {
                    match &state.current_tab_item {
                        TabItem::Bbsmenu => match &state.focus_left_or_right.get() {
                            // Category
                            LR::Left => {
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
                                let selected = state.category_list_state.selected();
                                let selected_category =
                                    state.category_list.get(selected.unwrap()).unwrap();
                                let board_list = state.board_list.clone();
                                board_list
                                    .lock()
                                    .unwrap()
                                    .replace(selected_category.list.clone());
                            }
                            // BoardList
                            LR::Right => {
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
                KeyCode::Left => state.focus_left_or_right.set(LR::Left),
                KeyCode::Right => state.focus_left_or_right.set(LR::Right),
                _ => {}
            },
            Event::Tick => {}
            Event::Init => {}
        }
    }

    Ok(())
}

fn render_board<'a>(state: &mut State) -> (List<'a>, List<'a>) {
    let category_list_state = state.category_list_state.clone();
    let category_list = state.category_list.clone();

    let thread_block = Block::default()
        .borders(Borders::all())
        .style(
            Style::default().fg(if state.focus_left_or_right.get() == LR::Left {
                Color::White
            } else {
                Color::Black
            }),
        )
        .title("BoardCategory")
        .border_type(BorderType::Plain);

    let category_items: Vec<ListItem> = category_list
        .iter()
        .map(|category| {
            ListItem::new(Span::styled(
                category.category.to_string(),
                Style::default().fg(Color::White),
            ))
        })
        .collect();

    let selected_category = category_list
        .get(category_list_state.selected().unwrap())
        .expect("")
        .clone();

    let category_list = List::new(category_items)
        .block(thread_block)
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    let board_list_state = state.board_list_state.clone();
    let board_block = Block::default()
        .borders(Borders::all())
        .style(
            Style::default().fg(if state.focus_left_or_right.get() == LR::Right {
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

    // カテゴリから板を選択

    let board_list = List::new(board_items).block(board_block).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    (category_list, board_list)
}
