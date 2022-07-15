mod stateful_bbsmenu_tree;

use std::{
    error::Error,
    io,
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

use termchan::get::bbsmenu::Bbsmenu;
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::{Backend, TermionBackend},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Frame,
    Terminal,
};
use tui_tree_widget::Tree;

use crate::stateful_bbsmenu_tree::StatefulBbsTree;

pub enum Event {
    Input(Key),
    Tick,
}

pub fn events(tick_rate: u64) -> mpsc::Receiver<Event> {
    let tick_rate = Duration::from_millis(tick_rate);
    let (tx, rx) = mpsc::channel();
    let keys_tx = tx.clone();
    thread::spawn(move || {
        let stdin = io::stdin();
        for key in stdin.keys().flatten() {
            if let Err(err) = keys_tx.send(Event::Input(key)) {
                eprintln!("{}", err);
                return;
            }
        }
    });
    thread::spawn(move || {
        loop {
            if let Err(err) = tx.send(Event::Tick) {
                eprintln!("{}", err);
                break;
            }
            thread::sleep(tick_rate);
        }
    });
    rx
}

pub struct App<'a> {
    pub should_quit:   bool,
    pub event_rx:      Receiver<Event>,
    pub bbsmenu_state: StatefulBbsTree<'a>,
}

impl<'a> App<'a> {
    pub fn new(tick_rate: u64) -> Self {
        let event_rx = events(tick_rate);
        let state = StatefulBbsTree::new();
        App {
            should_quit: false,
            event_rx,
            bbsmenu_state: state,
        }
    }
    pub fn on_tick(&mut self) {
    }
    pub fn on_event(&mut self) {
        match self.event_rx.recv().unwrap() {
            Event::Input(key) => {
                match key {
                    Key::Char('q') => self.should_quit = true,
                    Key::Down => self.bbsmenu_state.next(),
                    Key::Up => self.bbsmenu_state.previous(),
                    Key::Left => self.bbsmenu_state.close(),
                    Key::Right => {
                        if self.bbsmenu_state.state.selected().len() >= 2 {
                        } else {
                            self.bbsmenu_state.open()
                        }
                    }
                    _ => {}
                }
            }
            Event::Tick => self.on_tick(),
        }
    }
}

fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let area = f.size();

    let items = Tree::new(app.bbsmenu_state.items.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Tree Widget {:?}", app.bbsmenu_state.state)),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, area, &mut app.bbsmenu_state.state);
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App<'_>,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| draw(f, &mut app))?;
        app.on_event();
        if app.should_quit {
            return Ok(());
        }
    }
}

pub async fn run(tick_rate: u64) -> Result<(), Box<dyn Error>> {
    // setup terminal
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(tick_rate);
    let menu = Bbsmenu::new("https://menu.5ch.net/bbsmenu.json".to_string())?
        .get()
        .await?;
    app.bbsmenu_state = StatefulBbsTree::from(menu);

    // create app and run it
    run_app(&mut terminal, app).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    run(200).await.unwrap();
}
