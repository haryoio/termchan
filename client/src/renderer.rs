use std::{future, io::Write, process};

use anyhow::Result;
use futures::executor::block_on;
use termion::{
    raw::{IntoRawMode, RawTerminal},
    screen::{self, AlternateScreen},
};
use tui::{backend::TermionBackend, Terminal};

use crate::{application::App, ui};
pub struct Renderer<W: Write> {
    terminal: Terminal<TermionBackend<AlternateScreen<W>>>,
}

impl<W: Write> Renderer<W> {
    pub fn new(out: W) -> Result<Renderer<W>> {
        let backend = TermionBackend::new(AlternateScreen::from(out));
        let terminal = Terminal::new(backend)?;
        Ok(Renderer { terminal })
    }
    pub fn render(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|mut f| ui::draw(&mut f, app))?;
        Ok(())
    }
    pub fn exit(&mut self) -> Result<()> {
        self.terminal.show_cursor()?;
        self.terminal.clear()?;
        self.terminal.flush()?;
        process::exit(0);
        Ok(())
    }
}

impl<W: Write> Drop for Renderer<W> {
    fn drop(&mut self) {
        self.terminal.show_cursor().expect("Failed to show cursor");
        // self.terminal.clear();
        if std::thread::panicking() {
            self.terminal.clear().expect("Failed to clear screen");
        }
    }
}
