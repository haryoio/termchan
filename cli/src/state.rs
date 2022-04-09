use std::marker::PhantomData;

use termchan::controller::{board::Board, menu::BbsCategories, thread::Thread};

pub enum Layout {
    Bbsmenu,
    BoardCategory,
    BoardList,
    Board,
    ThreadList,
    Thread,
    PostMessage,
    CreateThread,
}

pub struct State {
    pub layout: Layout,
    pub bbs_categories: Vec<BbsCategories>,
    pub board_list: Vec<Board>,
    pub board: PhantomData<Board>,
    pub thread_list: Vec<Thread>,
    pub thread: PhantomData<Thread>,
}

impl State {
    pub fn new() -> Self {
        Self {
            layout: Layout::Bbsmenu,
            bbs_categories: Vec::new(),
            board_list: Vec::new(),
            board: PhantomData,
            thread_list: Vec::new(),
            thread: PhantomData,
        }
    }
}
