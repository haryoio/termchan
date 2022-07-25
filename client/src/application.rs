use chrono::TimeZone;
use chrono_tz::Asia::Tokyo;
use eyre::Result;
use termchan::get::{
    bbsmenu::{Bbsmenu, CategoryContent, CategoryItem},
    board::{Board, ThreadSubject},
    thread::{Thread, ThreadPost, ThreadResponse},
};

use crate::{
    config::Theme,
    event::Event,
    state::{
        layout::{LayoutState, Pane},
        tab::{LeftTabItem, RightTabItem, TabsState},
    },
    ui::stateful_list::StatefulList,
};

#[derive(Clone)]
pub struct App {
    pub message:          String,
    pub right_tabs:       TabsState<RightTabItem>,
    pub left_tabs:        TabsState<LeftTabItem>,
    pub right_pane_items: Vec<(String, String, ThreadResponse)>,

    pub theme:      Theme,
    pub layout:     LayoutState,
    pub bbsmenu:    StatefulList<String>,
    pub categories: StatefulList<CategoryItem>,
    pub category:   StatefulList<CategoryContent>,
    pub board:      StatefulList<ThreadSubject>,
    pub thread:     StatefulList<ThreadPost>,
}

impl App {
    pub fn new() -> Self {
        let left_tabs = TabsState::new(vec![LeftTabItem::Bbsmenu]);
        let right_tabs = TabsState::new(vec![]);
        let layout = LayoutState::new();
        let categories = StatefulList::with_items(vec![CategoryItem {
            category_name:    "".to_string(),
            category_content: vec![],
        }]);
        let board = StatefulList::with_items(vec![ThreadSubject {
            url:        "".to_string(),
            board_name: "".to_string(),
            name:       "".to_string(),
            id:         "".to_string(),
            count:      0,
            ikioi:      0.0,
            created_at: Tokyo.timestamp(0, 0),
        }]);
        let category = StatefulList::with_items(vec![CategoryContent {
            board_name: "".to_string(),
            url:        "".to_string(),
        }]);
        let right_pane_items = vec![("".to_string(), "".to_string(), ThreadResponse::default())];

        App {
            left_tabs,
            right_tabs,
            right_pane_items,
            layout,
            message: "".to_string(),
            theme: Theme::default(),
            bbsmenu: StatefulList::with_items(vec![
                "https://menu.5ch.net/bbsmenu.json".to_string(),
                "https://menu.2ch.sc/bbsmenu.html".to_string(),
            ]),
            categories,
            category,
            board,
            thread: StatefulList::with_items(vec![]),
        }
    }
}

// GET
impl App {
    pub async fn get_menu_item(&mut self) -> String {
        self.bbsmenu
            .items
            .get(self.bbsmenu.selected())
            .unwrap()
            .clone()
    }
    pub async fn get_categories(&self) -> Vec<CategoryItem> {
        self.categories.items.clone()
    }
    // pub async fn get_category(&self) -> Vec<CategoryContent> {
    //     let category = self.get_categories().await;
    //     if category.len() <= self.category.state.selected().unwrap() {
    //         return vec![CategoryContent {
    //             board_name: "".to_string(),
    //             url:        "".to_string(),
    //         }];
    //     }
    //     category[self.category.state.selected().unwrap()]
    //         .category_content
    //         .clone()
    // }
    // pub async fn get_board(&self) -> Vec<ThreadSubject> {
    //     self.board.items.clone()
    // }
    // pub async fn get_thread(&self) -> Vec<ThreadPost> {
    //     self.thread.items.clone()
    // }
}

impl App {
    pub async fn update(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Get => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Bbsmenu => self.update_bbsmenu().await,
                            LeftTabItem::Categories => self.update_categories().await?,
                            LeftTabItem::Category(..) => self.update_category().await?,
                            LeftTabItem::Board(..) => self.update_board().await?,
                            LeftTabItem::Settings => {}
                        }
                    }
                    Pane::Main => {
                        match self.right_tabs.get() {
                            RightTabItem::Thread(..) => self.update_thread().await?,
                        }
                    }
                }
                Ok(())
            }
            Event::Down => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Bbsmenu => self.bbsmenu.next(),
                            LeftTabItem::Categories => self.categories.next(),
                            LeftTabItem::Category(..) => self.category.next(),
                            LeftTabItem::Board(..) => self.board.next(),
                            _ => {}
                        }
                    }
                    Pane::Main => {
                        match self.right_tabs.get() {
                            RightTabItem::Thread(..) => self.thread.next(),
                        }
                    }
                }
                Ok(())
            }
            Event::Up => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Bbsmenu => self.bbsmenu.prev(),
                            LeftTabItem::Categories => self.categories.prev(),
                            LeftTabItem::Category(..) => self.category.prev(),
                            LeftTabItem::Board(..) => self.board.prev(),
                            _ => {}
                        }
                    }
                    Pane::Main => {
                        match self.right_tabs.get() {
                            RightTabItem::Thread(..) => self.thread.prev(),
                        }
                    }
                }
                Ok(())
            }
            Event::ScrollToTop => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Bbsmenu => self.bbsmenu.state.select(Some(0)),
                            LeftTabItem::Categories => self.categories.state.select(Some(0)),
                            LeftTabItem::Category(..) => self.category.state.select(Some(0)),
                            LeftTabItem::Board(..) => self.board.state.select(Some(0)),
                            _ => {}
                        }
                    }
                    Pane::Main => {
                        match self.right_tabs.get() {
                            RightTabItem::Thread(..) => self.thread.state.select(Some(0)),
                        }
                    }
                }
                Ok(())
            }
            Event::ScrollToBottom => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Bbsmenu => {
                                self.bbsmenu
                                    .state
                                    .select(Some(self.bbsmenu.items.len() - 1));
                            }
                            LeftTabItem::Categories => {
                                self.categories
                                    .state
                                    .select(Some(self.categories.items.len() - 1));
                            }

                            LeftTabItem::Category(..) => {
                                self.category
                                    .state
                                    .select(Some(self.category.items.len() - 1));
                            }
                            LeftTabItem::Board(..) => {
                                self.board.state.select(Some(self.board.items.len() - 1));
                            }
                            _ => {}
                        }
                    }
                    Pane::Main => {
                        match self.right_tabs.get() {
                            RightTabItem::Thread(..) => {
                                self.thread.state.select(Some(self.thread.items.len() - 1));
                            }
                        }
                    }
                }
                Ok(())
            }
            Event::RemoveHistory => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Bbsmenu => {}
                            _ => self.left_tabs.hidtory_remove(),
                        }
                        Ok(())
                    }
                    Pane::Main => {
                        match self.right_tabs.get() {
                            RightTabItem::Thread(_, url) => {
                                // tabsの中で現在選択中のタブのindexを取得する
                                if self.right_tabs.titles.len() >= 1 {
                                    return Ok(());
                                }
                                let idx = self
                                    .right_tabs
                                    .titles
                                    .iter()
                                    .position(|x| {
                                        // nameは被る可能性があるので、一意の値であるurlを使用して位置を取得
                                        match x {
                                            RightTabItem::Thread(.., url2) => &url == url2,
                                        }
                                    })
                                    .unwrap();
                                self.right_tabs.titles.remove(idx);
                                if self.right_tabs.index >= 1 {
                                    self.right_tabs.index -= 1;
                                }
                            }
                        }
                        Ok(())
                    }
                }
            }
            Event::Right => {
                if self.layout.focus_pane == Pane::Main {
                    self.thread.items = self.right_pane_items[self.right_tabs.index + 1]
                        .2
                        .clone()
                        .posts;
                    self.right_tabs.next();
                }
                Ok(())
            }
            Event::Left => {
                if self.layout.focus_pane == Pane::Main {
                    self.thread.items = self.right_pane_items[self.right_tabs.index + 1]
                        .2
                        .clone()
                        .posts;
                    self.right_tabs.previous();

                    // println!("{:?}", self.right_pane_items);
                }
                Ok(())
            }
            Event::Tab => Ok(()),
            Event::Enter => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Bbsmenu => {
                                self.layout.focus_pane = Pane::Side;
                                let _ = self.update_categories().await?;
                                self.left_tabs.history_add(LeftTabItem::Categories);
                                self.left_tabs.next();
                            }
                            LeftTabItem::Categories => {
                                {
                                    let _ = self.update_category().await;
                                    self.layout.focus_pane = Pane::Side;
                                }

                                let categ = self.get_categories().await;
                                let selected = self.categories.state.selected().unwrap();
                                if selected < categ.len() {
                                    {
                                        self.left_tabs.history_add(LeftTabItem::Category(
                                            categ[selected].clone().category_name,
                                        ));
                                    }
                                    self.left_tabs.next();
                                }
                            }
                            LeftTabItem::Category(..) => {
                                {
                                    let _ = self.update_board().await;
                                    self.layout.focus_pane = Pane::Side;
                                }

                                let categ = self.category.items
                                    [self.category.state.selected().unwrap()]
                                .clone();

                                self.left_tabs
                                    .history_add(LeftTabItem::Board(categ.board_name.clone()));
                                self.left_tabs.next();
                            }
                            LeftTabItem::Board(..) => {
                                {
                                    let _ = self.update_thread().await;
                                }
                                let idx = self.right_tabs.index;
                                let board = self.right_pane_items[idx].clone();

                                self.right_tabs
                                    .history_add(RightTabItem::Thread(board.0, board.1));

                                self.layout.toggle_focus_pane();
                                self.right_tabs.next();
                            }
                            LeftTabItem::Settings => {
                                self.layout.focus_pane = Pane::Main;
                                self.right_tabs.history_add(RightTabItem::Thread(
                                    "".to_string(),
                                    "".to_string(),
                                ));
                            }
                        }
                        Ok(())
                    }
                    Pane::Main => {
                        match self.right_tabs.get() {
                            RightTabItem::Thread(..) => {
                                // self.left_tabs.set(LeftTabItem::Bbsmenu);
                            }
                        }
                        Ok(())
                    }
                }
            }
            Event::Message(message) => {
                self.message = message.clone();
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub async fn update_bbsmenu(&mut self) {
        // let url = self.get_menu_item().await;
    }
    pub async fn update_categories(&mut self) -> Result<()> {
        let url = self.get_menu_item().await;
        let bbsmenu = Bbsmenu::new(url.clone())?.get().await?;
        self.categories.items = bbsmenu.menu_list;
        Ok(())
    }
    pub async fn update_category(&mut self) -> Result<()> {
        let categ = self
            .categories
            .items
            .get(self.categories.state.selected().unwrap())
            .unwrap();
        self.category.items = categ.category_content.clone();
        Ok(())
    }
    pub async fn update_board(&mut self) -> Result<()> {
        let board_item = self
            .category
            .items
            .get(self.category.state.selected().unwrap())
            .unwrap();
        let board = Board::new(board_item.url.clone())?.get().await?;
        self.board.items = board;
        Ok(())
    }
    pub async fn update_thread(&mut self) -> Result<()> {
        let thread_item = self
            .board
            .items
            .get(self.board.state.selected().unwrap())
            .unwrap();

        let thread = Thread::new(thread_item.url.clone())?.get().await;

        let thread = match thread {
            Ok(thread) => thread,
            Err(e) => {
                self.message = format!("{}", e);
                ThreadResponse::default()
            }
        };
        self.thread.items = thread.posts;
        Ok(())
    }
}
