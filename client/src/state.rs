use std::{
    cell::RefCell,
    collections::HashMap,
    f32::consts::E,
    fmt::Display,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::config::{Config, Theme};

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

            Self::Settings => write!(f, "Settings"),
        }
    }
}

impl Default for LeftTabItem {
    fn default() -> Self {
        LeftTabItem::Bbsmenu
    }
}

#[derive(Debug, Clone)]
pub enum RightTabItem {
    Thread(Title, Url),
}
impl Display for RightTabItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Thread(title, ..) => write!(f, "{}", title),
        }
    }
}

impl Default for RightTabItem {
    fn default() -> Self {
        RightTabItem::Thread("".to_string(), "".to_string())
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

#[derive(Debug, Clone)]
pub struct TabsState<T: Display + Default> {
    pub titles: Vec<T>,
    pub index:  usize,
}

impl<T> TabsState<T>
where
    T: Display + Default + Clone,
{
    pub fn new(titles: Vec<T>) -> TabsState<T> {
        TabsState { titles, index: 0 }
    }
    pub fn get(&self) -> T {
        if self.titles.len() > self.index {
            self.titles[self.index].clone()
        } else if self.titles.len() == 0 {
            T::default().clone()
        } else {
            self.titles[0].clone()
        }
    }
    pub fn history_add(&mut self, title: T) {
        self.titles.push(title);
    }
    pub fn hidtory_remove(&mut self) {
        if self.titles.len() >= 2 {
            self.titles.pop();
            if self.titles.len() <= self.index + 1 {
                self.index = self.titles.len() - 1;
            }
        }
    }

    pub fn next(&mut self) {
        self.index += 1;
        if self.index >= self.titles.len() {
            self.index = 0;
        }
    }

    pub fn previous(&mut self) {
        if self.index >= 1 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
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
