use std::cell::Cell;

use futures::io::Repeat;
use termchan::controller::{
    menu::{BbsCategories, BoardUrl},
    reply::Reply,
    thread::Thread as TCThread,
};

use crate::widgets::{
    atomic_stateful_list::AtomicStatefulList, reply_form::ReplyForm, stateful_list::StatefulList,
};

pub enum EventType<I> {
    Input(I),
    Tick,
}

#[derive(Debug, Clone)]
pub enum InputMode {
    Normal,
    Editing,
    Input,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabItem {
    Bbsmenu,
    Board,
    Settings,
}

// 左右ペインへの移動
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pane {
    Left,
    Right,
    //Resize,
}

#[derive(Debug, Clone)]
pub struct State {
    pub category: StatefulList<BbsCategories>,
    pub boards: AtomicStatefulList<BoardUrl>,
    pub threads: StatefulList<TCThread>,
    pub thread: AtomicStatefulList<Reply>,
    pub current_history: TabItem,
    pub history: Vec<TabItem>,
    pub board_url: String,
    pub focus_pane: Cell<Pane>,
    pub input_mode: InputMode,
    pub reply_form: ReplyForm,
}

impl State {
    pub fn new() -> Self {
        Self {
            category: StatefulList::with_items(vec![]),
            boards: AtomicStatefulList::with_items(vec![]),
            threads: StatefulList::with_items(vec![]),
            thread: AtomicStatefulList::with_items(vec![]),
            current_history: TabItem::Bbsmenu,
            history: Vec::new(),
            board_url: String::new(),
            focus_pane: Cell::new(Pane::Left),
            input_mode: InputMode::Normal,
            reply_form: ReplyForm::new(),
        }
    }
    pub fn current_category(&self) -> &BbsCategories {
        &self.category.items[self.category.state.selected().unwrap_or(0)]
    }

    pub fn current_board(&self) -> &BoardUrl {
        let selected_board = self.boards.state.selected().unwrap_or(0);
        &self.current_category().list[selected_board]
    }

    pub fn current_thread(&self) -> &TCThread {
        &self.threads.items[self.threads.state.selected().unwrap_or(0)]
    }

    pub fn current_reply(&self) -> &Reply {
        let selected_reply = self.threads.state.selected().unwrap();
        &self.current_thread().list[selected_reply]
    }

    pub fn add_history(&mut self, item: TabItem) {
        self.current_history = item.clone();
        self.history.push(item);
    }
}
