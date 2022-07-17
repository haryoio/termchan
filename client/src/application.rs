use std::{collections::HashMap, f32::consts::E, fmt::Display, sync::Arc, time::Duration};

use termchan::get::{
    bbsmenu::{Bbsmenu, CategoryContent, CategoryItem},
    board::{Board, ThreadSubject},
    thread::{Thread, ThreadPost, ThreadResponse},
};
use termion::event::Key;
use tokio::sync::Mutex;
use tui::widgets::{ListItem, ListState};

use crate::{
    config::{Config, Theme},
    event::Event,
    state::{LeftTabItem, RightTabItem},
};
#[derive(Debug, Clone)]
pub struct TabsState<T: Display> {
    pub titles: Vec<T>,
    pub index:  usize,
}

impl<T> TabsState<T>
where
    T: Display,
{
    pub fn new(titles: Vec<T>) -> TabsState<T> {
        TabsState { titles, index: 0 }
    }
    pub fn get(&self) -> &T {
        &self.titles[self.index]
    }
    pub fn history_add(&mut self, title: T) {
        self.titles.push(title);
    }
    pub fn hidtory_remove(&mut self) {
        self.titles.pop();
        if self.index >= self.titles.len() {
            self.index = self.titles.len() - 1;
        }
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

#[derive(Debug, Clone)]
pub struct StatefulMutexList<T> {
    pub state: ListState,
    pub items: Arc<Vec<Mutex<T>>>,
}

impl<T> StatefulMutexList<T>
where
    T: Clone,
{
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
    pub async fn update(&mut self, f: impl Fn(&mut T)) {
        for item in self.items.iter() {
            let mut item = item.lock().await;
            f(&mut item);
        }
    }
    pub async fn update_with_items(&mut self, items: Vec<T>) {
        let mut new_items: Vec<Mutex<T>> = Vec::new();
        for item in items {
            new_items.push(Mutex::new(item));
        }
        self.items = Arc::new(new_items);
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
    pub fn prev(&mut self) {
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
    pub async fn get(&self) -> Option<T> {
        Some(
            self.items
                .get(self.state.selected()?)
                .unwrap()
                .lock()
                .await
                .clone(),
        )
    }
    pub async fn get_index(&self, index: usize) -> Option<T> {
        Some(self.items.get(index).unwrap().lock().await.clone())
    }

    pub async fn to_list_items<'a>(
        &'a self,
        f: &dyn for<'b> Fn(&'b T) -> ListItem<'a>,
    ) -> Vec<ListItem<'a>> {
        let mut list_items: Vec<ListItem> = Vec::new();
        for (_i, item) in self.items.iter().enumerate() {
            let item = item.lock().await;
            list_items.push(f(&item));
        }
        list_items
    }
}

#[derive(Debug, Clone)]
pub struct LayoutState {
    pub visible_sidepane: bool,
    pub focus_pane:       Pane,
}
#[derive(PartialEq, Clone, Debug)]
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
#[derive(Clone)]
pub struct App<'a> {
    pub messages:   Arc<Mutex<Vec<&'a str>>>,
    pub right_tabs: TabsState<RightTabItem>,
    pub left_tabs:  TabsState<LeftTabItem>,

    pub theme:      Theme,
    pub layout:     LayoutState,
    pub bbsmenu:    StatefulMutexList<String>,
    pub categories: StatefulMutexList<CategoryItem>,
    pub category:   StatefulMutexList<CategoryContent>,
    pub board:      Arc<Mutex<CategoryContent>>,
    pub threads:    StatefulMutexList<ThreadSubject>,
    pub thread:     StatefulMutexList<ThreadPost>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let left_tabs = TabsState::new(vec![LeftTabItem::Bbsmenu]);
        let right_tabs = TabsState::new(vec![RightTabItem::Thread("".to_string())]);
        let layout = LayoutState::new();
        let categories = StatefulMutexList::with_items(vec![CategoryItem {
            category_name:    "".to_string(),
            category_content: vec![],
        }]);
        let threads = StatefulMutexList::with_items(vec![ThreadSubject {
            url:        "".to_string(),
            board_name: "".to_string(),
            name:       "".to_string(),
            id:         "".to_string(),
            count:      0,
        }]);
        let category = StatefulMutexList::with_items(vec![CategoryContent {
            board_name: "".to_string(),
            url:        "".to_string(),
        }]);

        App {
            left_tabs,
            right_tabs,

            layout,
            messages: Arc::new(Mutex::new(Vec::new())),
            theme: Theme::default(),
            bbsmenu: StatefulMutexList::with_items(vec![
                "https://menu.5ch.net/bbsmenu.json".to_string(),
                "https://menu.2ch.sc/bbsmenu.html".to_string(),
            ]),
            categories,
            category,
            board: Arc::new(Mutex::new(CategoryContent {
                board_name: "".to_string(),
                url:        "".to_string(),
            })),
            threads,
            thread: StatefulMutexList::with_items(vec![]),
        }
    }
}

// GET
impl<'a> App<'a> {
    pub async fn get_menu_item(&mut self) -> String {
        self.bbsmenu.get().await.unwrap().clone()
    }
    pub async fn get_categories(&self) -> Arc<Vec<Mutex<CategoryItem>>> {
        self.categories.items.clone()
    }
    pub async fn get_category(&self) -> Vec<CategoryContent> {
        let category = self.get_categories().await;
        if category.len() <= self.category.state.selected().unwrap() {
            return vec![CategoryContent {
                board_name: "".to_string(),
                url:        "".to_string(),
            }];
        }
        self.categories
            .get()
            .await
            .unwrap()
            .clone()
            .category_content
    }
    pub async fn get_board(&self) -> CategoryContent {
        self.board.lock().await.clone()
    }
    pub async fn get_threads(&self) -> Arc<Vec<Mutex<ThreadSubject>>> {
        self.threads.items.clone()
    }
    pub async fn get_thread(&self) -> Vec<ThreadPost> {
        let thread_idx = self.threads.state.selected().unwrap();
        let threads = self.thread.items.clone();
        let mut a = vec![];
        for thread in threads.iter() {
            a.push(thread.lock().await.clone());
        }
        a
    }
}

impl<'a> App<'a> {
    pub async fn update(&mut self, event: Event) {
        match event {
            Event::Get => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Bbsmenu => self.update_bbsmenu().await,
                            LeftTabItem::Categories => self.update_categories().await,
                            LeftTabItem::Category(..) => self.update_category().await,
                            LeftTabItem::Board(..) => self.update_board().await,
                            LeftTabItem::Settings => {
                                // self.update_settings().await;
                            }
                            _ => {}
                        }
                    }
                    Pane::Main => {
                        match self.right_tabs.get() {
                            RightTabItem::Thread(..) => self.update_thread().await,
                            _ => {}
                        }
                    }
                }
            }
            Event::Down => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Bbsmenu => self.bbsmenu.next(),
                            LeftTabItem::Categories => self.categories.next(),
                            LeftTabItem::Category(..) => self.category.next(),
                            LeftTabItem::Board(..) => {}
                            LeftTabItem::Threads => self.threads.next(),
                            _ => {}
                        }
                    }
                    Pane::Main => {
                        match self.right_tabs.get() {
                            RightTabItem::Thread(..) => self.thread.next(),
                        }
                    }
                }
            }
            Event::Up => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Bbsmenu => self.bbsmenu.prev(),
                            LeftTabItem::Categories => self.categories.prev(),
                            LeftTabItem::Category(..) => self.category.prev(),
                            LeftTabItem::Board(..) => {}
                            LeftTabItem::Threads => self.threads.prev(),
                            _ => {}
                        }
                    }
                    Pane::Main => {
                        match self.right_tabs.get() {
                            RightTabItem::Thread(..) => self.thread.prev(),
                        }
                    }
                }
            }
            Event::Tab => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        self.left_tabs.next();
                    }
                    Pane::Main => {
                        self.right_tabs.next();
                    }
                }
            }
            Event::Enter => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Bbsmenu => {
                                self.layout.focus_pane = Pane::Side;
                                self.update_categories().await;
                                self.left_tabs.history_add(LeftTabItem::Categories);
                                self.left_tabs.next();
                            }
                            LeftTabItem::Categories => {
                                self.layout.focus_pane = Pane::Side;
                                self.update_category().await;
                                let categ = self.get_categories().await;
                                let selected = self.categories.state.selected().unwrap();
                                if selected < categ.len() {
                                    self.left_tabs.history_add(LeftTabItem::Category(
                                        categ[selected].lock().await.clone().category_name,
                                    ));
                                    self.left_tabs.next();
                                }
                            }
                            LeftTabItem::Category(..) => {
                                self.layout.focus_pane = Pane::Side;
                                self.update_board().await;
                            }
                            LeftTabItem::Board(..) => {
                                self.layout.focus_pane = Pane::Main;
                                self.right_tabs
                                    .history_add(RightTabItem::Thread("".to_string()));
                            }
                            LeftTabItem::Settings => {
                                self.layout.focus_pane = Pane::Main;
                                self.right_tabs
                                    .history_add(RightTabItem::Thread("".to_string()));
                            }
                            _ => {}
                        }
                    }
                    Pane::Main => {
                        match self.right_tabs.get() {
                            RightTabItem::Thread(..) => {
                                self.layout.focus_pane = Pane::Side;
                                // self.left_tabs.set(LeftTabItem::Bbsmenu);
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub async fn update_bbsmenu(&mut self) {
        // let url = self.get_menu_item().await;
    }
    pub async fn update_categories(&mut self) {
        let url = self.get_menu_item().await;
        let bbsmenu = Bbsmenu::new(url.clone()).unwrap().get().await;

        self.categories
            .update_with_items(bbsmenu.unwrap().menu_list)
            .await;
    }
    pub async fn update_category(&mut self) {
        let category_item = self.categories.get().await.unwrap();
        self.category
            .update_with_items(category_item.category_content)
            .await;
    }
    pub async fn update_board(&mut self) {
    }
    pub async fn update_threads(&mut self) {
        let board = self.get_board().await.url;
        let board = Board::new(board).unwrap().get().await.unwrap();
        self.threads.update_with_items(board);
    }
    pub async fn update_thread(&mut self) {
        let thread_item = self.threads.get().await.unwrap();
        let thread = Thread::new(thread_item.url.clone())
            .unwrap()
            .get()
            .await
            .unwrap();
        self.thread.update_with_items(thread.posts).await;
    }
}
