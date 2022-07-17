/// flux architecture
///
/// move_up(&mut store);
/// move_dowm(&mut store);
/// store.update(|state| {
///    state.move_up();
/// });
use std::{cell::RefCell, collections::HashMap, fmt::Display};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::config::Theme;

/// UI
/// tab name / bredcrumb /

pub struct State {
    message:   Option<String>,
    tab_index: Mutex<Arc<usize>>,
    tab_items: Arc<Vec<Mutex<String>>>,
    theme:     Mutex<Arc<Theme>>,
}

#[derive(Debug, Clone)]
pub enum LeftTabItem {
    Bbsmenu,
    Categories,
    Category(Title),
    Board(Title),
    Threads,
    Settings,
}
type Title = String;
type Url = String;

impl Display for LeftTabItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bbsmenu => write!(f, "BBSmenu"),
            Self::Categories => write!(f, "Categories"),
            Self::Category(title) => write!(f, "{}", title),
            Self::Board(title) => write!(f, "{}", title),
            Self::Threads => write!(f, "Threads"),
            Self::Settings => write!(f, "Settings"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RightTabItem {
    Thread(Title),
}
impl Display for RightTabItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Thread(title) => write!(f, "{}", title),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tab<T> {
    pub tab_index: usize,
    pub tab_list:  Vec<T>,
}
impl<T> Tab<T>
where
    T: Display + Clone,
{
    pub fn new(tab_list: Vec<T>) -> Tab<T> {
        Tab {
            tab_index: 0,
            tab_list,
        }
    }
    pub fn get_tab(&self) -> T {
        self.tab_list[self.tab_index].clone()
    }
    pub fn next(&mut self) {
        self.tab_index += 1;
        if self.tab_index >= self.tab_list.len() {
            self.tab_index = 0;
        }
    }
    pub fn prev(&mut self) {
        self.tab_index -= 1;
        if self.tab_index >= self.tab_list.len() {
            self.tab_index = self.tab_list.len() - 1;
        }
    }
}
