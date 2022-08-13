use std::{error::Error, io, process};

use termion::raw::{IntoRawMode, RawTerminal};
use tui_textarea::Input;

use crate::{
    application::App,
    config::cache::CacheState,
    ctrl,
    database::logger::init_log,
    event::{event_sender, Command, Event},
    key,
    renderer::Renderer,
    state::layout::Pane,
};

pub async fn run() -> Result<(), Box<dyn Error>> {
    // setup terminal
    #[cfg(debug_assertions)]
    init_log("./termchan-tui.log".to_string())?;

    let mut render = Renderer::new(RawTerminal::from(io::stdout().into_raw_mode()?))?;
    info!("Renderer initialized");

    let cache = CacheState::get();
    let mut app = match cache {
        Some(app) => app,
        None => App::new(),
    };

    app.update(Event::Down).await?;
    app.update(Event::Up).await?;
    info!("State initialized");

    'main: loop {
        let _ = render.render(&mut app.clone());
        let mut rx = event_sender().await;

        while let Some(message) = rx.recv().await {
            use tui_textarea::Key::*;
            if !app.input_mode && !app.layout.visible_popup {
                match message {
                    Command::Event(evt) => {
                        match evt.into() {
                            ctrl!(Char('q')) => {
                                let _ = CacheState::set(app.clone());
                                render.exit()?;
                                break 'main;
                            }
                            key!(Char('j')) | key!(Down) => app.update(Event::Down).await?,
                            key!(Char('k')) | key!(Up) => app.update(Event::Up).await?,
                            key!(Char('h')) | key!(Left) => app.update(Event::Left).await?,
                            key!(Char('l')) | key!(Right) => app.update(Event::Right).await?,
                            key!(Char('\t')) => app.update(Event::Tab).await?,
                            key!(Char('r')) => {
                                app.update(Event::Get).await?;
                                match app.layout.focus_pane {
                                    Pane::Main => app.update(Event::ScrollToBottom).await?,
                                    Pane::Side => app.update(Event::ScrollToTop).await?,
                                    _ => (),
                                }
                            }
                            ctrl!(Char('f')) => app.update(Event::ToggleFilter).await?,
                            key!(Char('f')) => app.update(Event::ToggleBookmark).await?,
                            key!(Enter) => {
                                app.update(Event::Enter).await?;
                                app.update(Event::ScrollToTop).await?;
                            }
                            key!(Backspace) => app.update(Event::RemoveHistory).await?,
                            key!(Esc) => app.update(Event::ClosePopup).await?,
                            _ => (),
                        }

                        let _ = CacheState::set(app.clone());
                        let _ = render.render(&mut app.clone());
                    }

                    Command::Tick => {}
                    _ => {}
                }
            } else if app.layout.visible_popup && !app.input_mode {
                match message {
                    Command::Event(evt) => {
                        match evt.into() {
                            key!(Esc) => app.update(Event::ClosePopup).await?,
                            key!(Char('\t')) => app.update(Event::ToggleTextArea).await?,
                            ctrl!(Char('s')) => app.update(Event::Post).await?,
                            key!(Enter) => app.update(Event::EnableInputMode).await?,
                            _ => (),
                        }

                        let _ = CacheState::set(app.clone());
                        let _ = render.render(&mut app.clone());
                    }
                    Command::Tick => {}
                    _ => {}
                }
            } else {
                match message {
                    Command::Event(event) => {
                        match event.into() {
                            key!(Esc) => app.update(Event::DisableInputMode).await?,
                            ctrl!(Char('x')) => app.update(Event::ToggleTextArea).await?,
                            input => app.update(Event::Input(input)).await?,
                        }
                        let _ = render.render(&mut app.clone());
                    }
                    Command::Tick => {}
                    _ => {}
                }
            }
        }
    }

    let _ = CacheState::set(app);
    process::exit(0);
}
