mod application;
mod config;
mod event;
mod renderer;
mod state;
mod style;
use std::{
    error::Error,
    io::{self, Write},
    process,
    sync::Arc,
    time::Duration,
};

use application::App;
use event::{event_handler, event_sender, Command};
use futures::executor::{block_on, Enter};
use renderer::Renderer;
use termion::{
    async_stdin,
    event::{parse_event, Event as TermionEvent, Key},
    input::{MouseTerminal, TermRead},
    raw::{IntoRawMode, RawTerminal},
};
use tokio::sync::{
    mpsc::{self, Receiver},
    Mutex,
};
mod ui;

use crate::event::Event;
#[tokio::main]
async fn main() {
    let _ = run().await;
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    // setup terminal
    let mut render = renderer::Renderer::new(RawTerminal::from(io::stdout().into_raw_mode()?))?;
    let mut app = App::new();

    loop {
        let mut rx = event_sender().await;

        use Command::*;
        while let Some(message) = rx.recv().await {
            match message {
                Exit => todo!(),
                Input(key) => {
                    use termion::event::Key::*;
                    match key {
                        Char('q') => process::exit(0),
                        Ctrl('b') => app.layout.toggle_visible_sidepane(),
                        Char('\t') => app.layout.toggle_focus_pane(),
                        Char('l') => app.update(Event::Get).await,
                        Char('t') => app.update(Event::Tab).await,
                        Char('j') => app.update(Event::Down).await,
                        Char('k') => app.update(Event::Up).await,
                        Char('\n') => {
                            app.update(Event::Enter).await;
                        }
                        _ => {}
                    }
                    let _ = render.render(&mut app.clone()).await;
                }
                Tick => {}
                _ => {}
            }
        }
    }
}
