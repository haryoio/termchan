use std::{io, time::Duration};

use derive_more::Display;
use termion::{
    event::{Event as TermionEvent, Key},
    input::TermRead,
    raw::IntoRawMode,
};
use tokio::sync::mpsc::{self, Receiver};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    Exit,
    Input(Key),
    Tick,
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
}
// send event to event_handler
pub async fn event_sender() -> Receiver<Command> {
    let (tx, rx) = mpsc::channel(10);
    let key_tx = tx.clone();
    tokio::spawn(async move {
        let tx = key_tx.clone();
        let _raw_term = std::io::stdout().into_raw_mode().unwrap();
        let stdin = io::stdin();
        for evt in stdin.events().map(|evt| evt.unwrap()) {
            match evt {
                TermionEvent::Key(key) => {
                    let _ = tx.send(Command::Input(key)).await;
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

#[derive(Debug, Clone, Display)]
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

#[derive(Debug, Clone, Display)]
pub enum Order {
    #[display(fmt = "昇順")]
    Asc,
    #[display(fmt = "降順")]
    Desc,
}
