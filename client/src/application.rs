use std::{
    collections::HashMap,
    f32::consts::E,
    sync::{mpsc::Receiver, Arc, Mutex},
    time::Duration,
};

use termchan::get::{board::ThreadSubject, thread::ThreadPost};
use termion::event::Key;
use tui::widgets::ListState;

use crate::{
    config::{Config, Theme},
    event::{events, Event},
};
pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index:  usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct StatefulMutexList<T> {
    pub state: ListState,
    pub items: Arc<Vec<Mutex<T>>>,
}

impl<T> StatefulMutexList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulMutexList<T> {
        let mut new_items: Vec<Mutex<T>> = Vec::new();
        for item in items {
            new_items.push(Mutex::new(item));
        }
        StatefulMutexList {
            state: ListState::default(),
            items: Arc::new(new_items),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct LayoutState {
    pub visible_sidepane: bool,
    pub focus_pane:       Pane,
}
#[derive(PartialEq)]
pub enum Pane {
    Side,
    Main,
}

impl LayoutState {
    pub fn new() -> LayoutState {
        LayoutState {
            visible_sidepane: true,
            focus_pane:       Pane::Side,
        }
    }
    pub fn toggle_visible_sidepane(&mut self) {
        if self.visible_sidepane {
            self.focus_pane = Pane::Main;
        }
        self.visible_sidepane = !self.visible_sidepane;
    }
    pub fn toggle_focus_pane(&mut self) {
        self.focus_pane = match self.focus_pane {
            Pane::Side => Pane::Main,
            Pane::Main => {
                if self.visible_sidepane {
                    Pane::Side
                } else {
                    Pane::Main
                }
            }
        };
    }
}

pub type ThreadSubjectList = Vec<Mutex<ThreadSubject>>;
pub type ThreadPostList = Vec<Mutex<ThreadPost>>;
pub struct App<'a> {
    pub messages:    Arc<Mutex<Vec<&'a str>>>,
    pub right_tabs:  TabsState<'a>,
    pub left_tabs:   TabsState<'a>,
    pub boards:      StatefulMutexList<ThreadSubjectList>,
    pub threads:     StatefulMutexList<ThreadPostList>,
    pub should_quit: bool,
    pub theme:       Theme,
    pub config:      Config,
    pub layout:      LayoutState,
    pub event_rx:    Receiver<Event>,
}

impl<'a> App<'a> {
    pub fn new(tick_rate: u64) -> Self {
        let left_tabs = TabsState::new(vec!["サーバ一覧", "板一覧", "設定"]);
        let right_tabs = TabsState::new(vec!["なし"]);
        let layout = LayoutState::new();
        let boards = StatefulMutexList::with_items(Vec::new());
        let threads = StatefulMutexList::with_items(Vec::new());
        let event_rx = events(tick_rate);
        App {
            left_tabs,
            right_tabs,
            boards,
            threads,
            should_quit: false,
            layout,
            messages: Arc::new(Mutex::new(Vec::new())),
            theme: Theme::default(),
            config: Config::default(),
            event_rx,
        }
    }
    pub fn on_tick(&mut self) {
    }
    pub fn on_event(&mut self) {
        match self.event_rx.recv().unwrap() {
            Event::Input(key) => {
                match key {
                    Key::Char('q') => self.should_quit = true,
                    Key::Ctrl('b') => self.layout.toggle_visible_sidepane(),
                    Key::Char('\t') => self.layout.toggle_focus_pane(),
                    _ => {}
                }
            }
            Event::Tick => self.on_tick(),
        }
    }
}

// struct Command<'a> {
//     pub name:        &'a str,
//     pub description: &'a str,
//     pub action:      fn(&mut App),
// }

// trait CommandHandler {
//     fn register(&mut self, key: Key, callback: fn(&mut App)) {};
// }
