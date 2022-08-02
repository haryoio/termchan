use std::{io, time::Duration};

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

// pub async fn event_handler<'a>(rx: Arc<Mutex<Receiver<Command>>>, app: &mut App) {
//     use Command::*;
//     let mut rx = rx.lock().await;
//     while let Some(message) = rx.recv().await {
//         match message {
//             Input(key) => {
//                 use termion::event::Key::*;
//                 match key {
//                     Char('q') => process::exit(0),
//                     Ctrl('b') => app.layout.toggle_visible_sidepane(),
//                     Char('\t') => app.layout.toggle_focus_pane(),
//                     Char('l') => app.update(Event::Get).await,
//                     Char('c') => println!("{:?}", app.category),
//                     Char('t') => app.update(Event::Tab).await,
//                     Char('j') | Down => app.update(Event::Down).await,
//                     Char('k') | Up => app.update(Event::Up).await,
//                     _ => {}
//                 }
//             }
//             Tick => {}
//             _ => unimplemented!(),
//         }
//     }
// }
