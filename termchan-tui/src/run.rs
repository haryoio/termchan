use std::{
    error::Error,
    io::{self, Write},
    process,
};

use termion::raw::{IntoRawMode, RawTerminal};

use crate::{
    application::App,
    event::{event_sender, Command, Event},
    renderer::Renderer,
    state::layout::Pane,
};

pub async fn run() -> Result<(), Box<dyn Error>> {
    // setup terminal
    let mut render = Renderer::new(RawTerminal::from(io::stdout().into_raw_mode()?))?;
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
                        Char('\t') => app.layout.next(),
                        BackTab => app.layout.prev(),
                        Char('r') => {
                            app.update(Event::Get).await?;
                            match app.layout.focus_pane {
                                Pane::Main => app.update(Event::ScrollToBottom).await?,
                                Pane::Side => app.update(Event::ScrollToTop).await?,
                                _ => (),
                            }
                        }
                        Char('f') => {
                            app.update(Event::ToggleBookmark).await?;
                        }
                        Ctrl('f') => {
                            app.update(Event::ToggleFilter).await?;
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
