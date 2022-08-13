use std::{io, time::Duration};

use derive_more::Display;
use serde::{Deserialize, Serialize};
use termion::{
    event::{Event as TermionEvent, Key},
    input::TermRead,
    raw::IntoRawMode,
};
use tokio::sync::mpsc::{self, Receiver};
use tui_textarea::Input;

#[macro_export]
macro_rules! ctrl {
    ($key:pat) => {
        Input {
            key:  $key,
            ctrl: true,
            alt:  false,
        }
    };
    () => {};
}
#[macro_export]
macro_rules! key {
    ($key:pat) => {
        Input {
            key:  $key,
            ctrl: false,
            alt:  false,
        }
    };
}
#[macro_export]
macro_rules! alt {
    ($key:pat) => {
        Input {
            key:  $key,
            ctrl: false,
            alt:  true,
        }
    };
    () => {};
}
#[macro_export]
macro_rules! ctrl_alt {
    ($key:pat) => {
        Input {
            key:  $key,
            ctrl: true,
            alt:  true,
        }
    };
    () => {};
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    Exit,
    Input(Key),
    Tick,
    Event(TermionEvent),
}

#[allow(dead_code)]
pub enum Event {
    Get,
    Post,
    Tab,
    Exit,
    Up,
    Down,
    Enter,
    Left,
    Right,
    RemoveHistory,
    ToggleSidepane,
    ToggleFocusPane,
    FocusNextPane,
    FocusPrevPane,
    ToggleBookmark,
    ToggleFilter,
    BackTab,
    NextTab,
    ScrollToTop,
    ScrollToBottom,
    Message(String),
    ClosePopup,
    ToggleInputMode,
    EnableInputMode,
    DisableInputMode,
    ToggleTextArea,
    Input(Input),
}
// send event to event_handler
pub async fn event_sender() -> Receiver<Command> {
    let (tx, rx) = mpsc::channel(10);
    let key_tx = tx.clone();
    tokio::spawn(async move {
        let tx = key_tx.clone();
        let _raw_term = std::io::stdout().into_raw_mode().unwrap();
        let stdin = io::stdin();
        // マウスのスクロールイベントが３連続で発生するのでiカウントして3回溜まったらttイベントを発生させる
        let mut scrollup_count = 0;
        let mut scrolldown_count = 0;
        let mut before_click_time = std::time::Instant::now();
        for evt in stdin.events().map(|evt| evt.unwrap()) {
            let ev = evt.clone();
            match ev {
                TermionEvent::Key(_) => {
                    // マウスの中ボタンを押したとき、Enterが連続で発生するのを防ぐ
                    let now = std::time::Instant::now();
                    if before_click_time.elapsed().as_millis() - now.elapsed().as_millis() > 500 {
                        if let Err(e) = tx.send(Command::Event(evt)).await {
                            eprintln!("{}", e);
                            break;
                        }
                    } else {
                        before_click_time = std::time::Instant::now();
                    }
                }
                // マウスのuスクロールイベントはTermionではUnsupportedとなっている
                // TODO! Windows, Macでのo挙動を確認する
                TermionEvent::Unsupported(s) => {
                    match s.clone().as_slice() {
                        [0x1b, 0x4f, 0x42] => {
                            if scrolldown_count >= 3 {
                                tx.send(Command::Event(TermionEvent::Key(Key::Down)))
                                    .await
                                    .unwrap();
                                scrolldown_count = 0;
                            } else {
                                scrolldown_count += 1;
                            }
                        }
                        [0x1b, 0x4f, 0x41] => {
                            if scrollup_count >= 3 {
                                tx.send(Command::Event(TermionEvent::Key(Key::Up)))
                                    .await
                                    .unwrap();
                                scrollup_count = 0;
                            } else {
                                scrollup_count += 1;
                            }
                        }
                        a => {
                            panic!("unsupported {:?}", a);
                        }
                    }
                }

                _ => {}
            }
        }
    });
    let tick_tx = tx.clone();
    tokio::spawn(async move {
        let tx = tick_tx.clone();
        loop {
            let mut interval = tokio::time::interval(Duration::from_millis(200));
            let _ = tx.send(Command::Tick).await;
            let _ = interval.tick().await;
        }
    });
    rx
}

#[derive(Debug, Clone, Display, Serialize, Deserialize)]
pub enum Sort {
    #[display(fmt = "スレ順({})", _0)]
    None(Order),
    #[display(fmt = "勢い({})", _0)]
    Ikioi(Order),
    #[display(fmt = "新しい({})", _0)]
    Latest(Order),
    #[display(fmt = "既読({})", _0)]
    AlreadyRead(Order),
}

impl Default for Sort {
    fn default() -> Self {
        Sort::None(Order::Asc)
    }
}

#[derive(Debug, Clone, Display, Serialize, Deserialize)]
pub enum Order {
    #[display(fmt = "昇順")]
    Asc,
    #[display(fmt = "降順")]
    Desc,
}
