extern crate cli;

use cli::{
    renderer::Renderer,
    state::{EventType, InputMode, Pane, State, TabItem},
};
use futures::executor::block_on;
use std::{
    io::{self, Write},
    time::Duration,
    vec,
};
use tokio::{sync::mpsc, time::Instant};

use anyhow::Context;
use cli::utils::Result;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use pprof;
use termchan::{
    configs::config::Config,
    controller::{
        board::Board,
        menu::{BbsCategories, BbsMenu, BoardUrl},
        reply::Reply,
        thread::Thread as TCThread,
    },
    sender::Sender,
};
#[tokio::main]
async fn main() -> Result<()> {
    // let guard = pprof::ProfilerGuardBuilder::default()
    //     .frequency(1000)
    //     .blocklist(&["libc", "libgcc", "pthread"])
    //     .build()
    //     .unwrap();

    let stdout = io::stdout();
    let mut state = State::new();
    let mut renderer = Renderer::new(stdout)?;

    // 設定を読み込み
    let config = Config::load();
    let bbsmenu_url = match config.unwrap().bbsmenu.url.first() {
        Some(url) => url.to_owned(),
        None => panic!("BBSMENU URLを設定してください。"),
    };

    state.category.items = BbsMenu::new(bbsmenu_url.to_string())
        .load()
        .await
        .context("Failed to load BBSMENU")?
        .clone();
    state.boards.set_items(state.category.items[0].list.clone());
    state.threads.items = vec![TCThread::default()];
    state.thread.set_items(vec![Reply::default()]);
    state.history = vec![TabItem::Bbsmenu];

    // TODO InputWidgetで置き換える
    // let block = Block::default().borders(Borders::ALL).title("Input");
    // let text = Text::from(Spans::from(Span::styled(
    //     "input",
    //     Style::default().fg(Color::Yellow),
    // )));
    // let para = Paragraph::new(text).block(block).wrap(Wrap { trim: false });

    let (tx, mut rx) = mpsc::channel(1);
    let tick_rate = Duration::from_millis(200);
    tokio::spawn(async move {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_millis(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("read works") {
                    if let Ok(_) = tx.send(EventType::Input(key)).await {}
                }
            }
            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(EventType::Tick).await {
                    last_tick = Instant::now();
                }
            }
        }
    });

    loop {
        let current_tab = &state.history.last().unwrap();

        renderer.render(&mut state.clone())?;

        match rx.recv().await.unwrap() {
            EventType::Input(event) => {
                {
                    match state.input_mode {
                        InputMode::Normal => {
                            match event.code {
                                KeyCode::Char('q') => {
                                    break;
                                }
                                KeyCode::Up => {
                                    match &current_tab {
                                        TabItem::Bbsmenu => {
                                            match &state.focus_pane.get() {
                                                // Category
                                                Pane::Left => {
                                                    state.category.previous();
                                                    let selected_category =
                                                        state.current_category();
                                                    let mut state = state.clone();
                                                    state
                                                        .boards
                                                        .set_items(selected_category.list.clone());
                                                } // Down -> Bbsmenu -> Pane::Left
                                                // BoardList
                                                Pane::Right => {
                                                    state.boards.previous();
                                                }
                                            }
                                        }
                                        TabItem::Board => {
                                            match &state.focus_pane.get() {
                                                // ThreadList
                                                Pane::Left => {
                                                    state.threads.previous();
                                                } // Down -> ThreadList -> Pane::Left
                                                // Thread
                                                Pane::Right => {
                                                    let selected = state.thread.state.selected();
                                                    if selected.is_some() {
                                                        state.thread.state.select(
                                                            selected.and_then(|i| {
                                                                if i <= 0 {
                                                                    Some(0)
                                                                } else {
                                                                    Some(i - 1)
                                                                }
                                                            }),
                                                        );
                                                    }
                                                } // Down -> Thread -> Pane::Right
                                            }
                                        }
                                        TabItem::Settings => todo!(),
                                    };
                                }
                                KeyCode::Down => {
                                    match &current_tab {
                                        TabItem::Bbsmenu => {
                                            match &state.focus_pane.get() {
                                                // Category
                                                Pane::Left => {
                                                    state.category.next();
                                                    let selected_category =
                                                        state.current_category();
                                                    let mut state = state.clone();
                                                    {
                                                        state.boards.set_items(
                                                            selected_category.list.clone(),
                                                        );
                                                    }
                                                }
                                                // BoardList
                                                Pane::Right => state.boards.next(),
                                            }
                                        }
                                        TabItem::Board => {
                                            match &state.focus_pane.get() {
                                                // ThreadList
                                                Pane::Left => state.threads.next(),
                                                // Thread
                                                Pane::Right => state.thread.next(),
                                            }
                                        }
                                        TabItem::Settings => todo!(),
                                    };
                                }

                                KeyCode::Enter => {
                                    match &current_tab {
                                        // 板を選択,スレッド一覧画面へ移行
                                        TabItem::Bbsmenu => {
                                            match state.focus_pane.get() {
                                                Pane::Left => state.focus_pane.set(Pane::Right),
                                                Pane::Right => {
                                                    // 選択した板URLを取得
                                                    state.board_url =
                                                        state.current_board().url.clone();
                                                    let new_threads =
                                                        Board::new(state.clone().board_url)
                                                            .load()
                                                            .await
                                                            .unwrap();
                                                    state.threads.items = new_threads;
                                                    state.focus_pane.set(Pane::Left);
                                                    state.add_history(TabItem::Board);
                                                }
                                            }
                                        }
                                        TabItem::Board => match state.focus_pane.get() {
                                            Pane::Left => {
                                                let mut thread = state.current_thread().clone();
                                                let reply_list = thread.load().await.unwrap();
                                                state.focus_pane.set(Pane::Right);
                                                state.thread.state.select(Some(0));
                                                state.thread.set_items(reply_list);
                                            }
                                            Pane::Right => {}
                                        },
                                        TabItem::Settings => todo!(),
                                    };
                                }
                                // resizemode
                                // ペインの比率を変更する
                                KeyCode::Char('R') => {}
                                KeyCode::Left => match state.focus_pane.get() {
                                    Pane::Left => match current_tab {
                                        TabItem::Bbsmenu => {
                                            state.focus_pane.set(Pane::Right);
                                        }
                                        TabItem::Board => {
                                            state.history.pop();
                                            state.focus_pane.set(Pane::Right);
                                        }
                                        TabItem::Settings => todo!(),
                                    },
                                    Pane::Right => {
                                        state.focus_pane.set(Pane::Left);
                                    }
                                },
                                KeyCode::Right => match state.focus_pane.get() {
                                    Pane::Left => state.focus_pane.set(Pane::Left),
                                    Pane::Right => {
                                        state.focus_pane.set(Pane::Right);
                                    }
                                },
                                KeyCode::Char('i') => {
                                    if current_tab == &&TabItem::Board
                                        && state.focus_pane.get() == Pane::Right
                                    {
                                        state.reply_form.toggle();
                                        match state.input_mode {
                                            InputMode::Normal => {
                                                state.input_mode = InputMode::Input;
                                            }
                                            InputMode::Input => {
                                                state.input_mode = InputMode::Normal;
                                            }
                                            InputMode::Editing => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        InputMode::Editing => match event.code {
                            KeyCode::Esc => {
                                state.input_mode = InputMode::Input;
                            }
                            KeyCode::Char(c) => {
                                block_on(state.reply_form.char(c));
                            }
                            KeyCode::Backspace => {
                                block_on(state.reply_form.backspace());
                            }
                            KeyCode::Enter => {
                                block_on(state.reply_form.enter());
                            }
                            KeyCode::Left => {
                                block_on(state.reply_form.left());
                            }
                            KeyCode::Right => {
                                block_on(state.reply_form.right());
                            }
                            KeyCode::Up => {
                                block_on(state.reply_form.up());
                            }
                            KeyCode::Down => {
                                block_on(state.reply_form.down());
                            }
                            _ => {}
                        },
                        InputMode::Input => match event.code {
                            KeyCode::Tab | KeyCode::Down => {
                                state.reply_form.next_form().await;
                            }
                            KeyCode::BackTab | KeyCode::Up => {
                                state.reply_form.prev_form().await;
                            }
                            KeyCode::Enter => {
                                if state.reply_form.focused() == 3 {
                                    let thread = state.current_thread();
                                    let sender = Sender::new(&thread);

                                    let mut state = state.clone();
                                    let message = state.reply_form.message().await;
                                    let name = state.reply_form.name().await;
                                    let mail = state.reply_form.mail().await;

                                    let res = sender
                                        .send(&message, Some(&name), Some(&mail))
                                        .await
                                        .unwrap();
                                    println!("{:?}", res);
                                } else {
                                    state.input_mode = InputMode::Editing;
                                }
                            }
                            KeyCode::Esc => {
                                state.input_mode = InputMode::Normal;
                                state.reply_form.toggle();
                            }
                            _ => {}
                        },
                    }
                }
            }

            EventType::Tick => {}
        }
    }

    // match guard.report().build() {
    //     Ok(report) => {
    //         let file = File::create("flamegraph.svg").unwrap();
    //         let mut options = pprof::flamegraph::Options::default();
    //         options.image_width = Some(10000);
    //         report.flamegraph_with_options(file, &mut options).unwrap();

    //         println!("report: {:?}", &report);
    //     }
    //     Err(_) => {}
    // };
    Ok(())
}
