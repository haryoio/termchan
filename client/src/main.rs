mod application;
mod config;
mod event;
mod renderer;
mod state;
mod style;
mod ui;
use std::{
    error::Error,
    io,
    sync::mpsc::{self, Receiver},
    time::Duration,
};

use application::App;
use event::{events, key_event_handler};
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::{Backend, TermionBackend},
    Terminal,
};

fn main() {
    run(200).unwrap();
}

pub fn run(tick_rate: u64) -> Result<(), Box<dyn Error>> {
    // setup terminal
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(tick_rate);

    // create app and run it
    run_app(&mut terminal, app)?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        app.on_event();
        if app.should_quit {
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() -> Result<(), Box<dyn Error>> {
        // setup terminal
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut app = App::new(200);
        loop {
            terminal.draw(|f| ui::draw_bbsmenu_tree(f, &mut app))?;
            app.on_event();
            if app.should_quit {
                return Ok(());
            }
        }
    }
}
