use std::{
    io,
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

use termion::{event::Key, input::TermRead};

use crate::application::App;

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

pub fn key_event_handler(app: &mut App) {
    let recv = &app.event_rx;
    match recv.recv().unwrap() {
        Event::Input(key) => {
            match key {
                Key::Char('q') => app.should_quit = true,
                Key::Ctrl('b') => app.layout.toggle_visible_sidepane(),
                Key::Char('\t') => app.layout.toggle_focus_pane(),
                _ => {}
            }
        }
        Event::Tick => app.on_tick(),
    }
}
