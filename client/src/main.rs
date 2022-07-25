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
};

use application::App;
use event::{event_sender, Command};
use termion::raw::{IntoRawMode, RawTerminal};

mod ui;

fn append_file(content: &str) {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("./debug.log")
        .unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
use crate::{event::Event, state::layout::Pane};
#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(e) => {
            append_file(&format!("{:?}", e));
            process::exit(1);
        }
    }
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    // setup terminal
    let mut render = renderer::Renderer::new(RawTerminal::from(io::stdout().into_raw_mode()?))?;
    let mut app = App::new();
    app.update(Event::Down).await?;

    loop {
        let _ = render.render(&mut app.clone());
        let mut rx = event_sender().await;

        use Command::*;
        while let Some(message) = rx.recv().await {
            match message {
                Input(key) => {
                    use termion::event::Key::*;
                    match key {
                        Char('q') => render.exit()?,
                        Ctrl('b') => app.layout.toggle_visible_sidepane(),
                        Char('\t') => app.layout.toggle_focus_pane(),
                        Char('r') => {
                            app.update(Event::Get).await?;
                            match app.layout.focus_pane {
                                Pane::Main => app.update(Event::ScrollToBottom).await?,
                                Pane::Side => app.update(Event::ScrollToTop).await?,
                            }
                        }
                        Ctrl('j') | Char('g') => app.update(Event::ScrollToBottom).await?,
                        Ctrl('k') | Char('G') => app.update(Event::ScrollToTop).await?,
                        Char('j') | Down => app.update(Event::Down).await?,
                        Char('k') | Up => app.update(Event::Up).await?,
                        Char('h') | Left => app.update(Event::Left).await?,
                        Char('l') | Right => app.update(Event::Right).await?,
                        Backspace => app.update(Event::RemoveHistory).await?,
                        Char('\n') => {
                            app.update(Event::Enter).await?;
                            app.update(Event::ScrollToTop).await?;
                        }
                        _ => {}
                    }
                    let _ = render.render(&mut app.clone());
                }
                Tick => {}
                _ => {}
            }
        }
    }
}
