use std::{future, io::Write};

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
    pub async fn render(&mut self, app: &mut App<'_>) -> Result<()> {
        self.terminal.draw(|mut f| ui::draw(&mut f, app))?;
        Ok(())
    }
}

impl<W: Write> Drop for Renderer<W> {
    fn drop(&mut self) {
        self.terminal.show_cursor().expect("Failed to show cursor");

        if std::thread::panicking() {
            eprintln!(
                "termchat paniced, to log the error you can redirect stderror to a file, example: termchat 2> termchat_log",
            );
        }
    }
}
