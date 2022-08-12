use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum LeftTabItem {
    Home,
    Bookmarks,
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
            Self::Home => write!(f, "Home"),
            Self::Bookmarks => write!(f, "お気に入り"),
            Self::Bbsmenu => write!(f, "板"),
            Self::Categories => write!(f, "カテゴリ"),
            Self::Category(title) => write!(f, "{}", title),
            Self::Board(title) => write!(f, "{}", title),
            Self::Settings => write!(f, "設定"),
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

    pub fn get_current(&self) -> T {
        self.titles[self.index].clone()
    }
}
