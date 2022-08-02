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
        bbsmenu::BbsMenuStateItem,
        board::BoardStateItem,
        bookmark::BookmarkStateItem,
        categories::CategoriesStateItem,
        home::{HomeItem, HomeStateItem},
        layout::{LayoutState, Pane},
        post::ThreadPostStateItem,
        tab::{LeftTabItem, RightTabItem, TabsState},
        thread::ThreadStateItem,
    },
    ui::stateful_list::StatefulList,
};

#[derive(Clone)]
pub struct App {
    pub message:    String,
    pub right_tabs: TabsState<RightTabItem>,
    pub left_tabs:  TabsState<LeftTabItem>,

    pub theme:      Theme,
    pub layout:     LayoutState,
    pub home:       StatefulList<HomeStateItem>,
    pub bookmark:   StatefulList<BookmarkStateItem>,
    pub bbsmenu:    StatefulList<BbsMenuStateItem>,
    pub categories: StatefulList<CategoriesStateItem>,
    pub category:   StatefulList<BoardStateItem>,
    pub board:      StatefulList<ThreadStateItem>,
    pub thread:     StatefulList<ThreadPostStateItem>,
}

impl App {
    pub fn new() -> Self {
        let left_tabs = TabsState::new(vec![LeftTabItem::Home]);
        let right_tabs = TabsState::new(vec![]);

        let layout = LayoutState::new();

        // BBS Menuを DBに登録。
        futures::executor::block_on(BbsMenuStateItem::init(vec![
            "https://menu.5ch.net/bbsmenu.json".to_string(),
            "https://menu.open2ch.net/bbsmenu.html".to_string(),
        ]));

        //  DB中のMenuを取得。
        let init_bbsmenu = futures::executor::block_on(BbsMenuStateItem::get()).unwrap();
        let bbsmenu = StatefulList::with_items(init_bbsmenu);
        let categories = StatefulList::with_items(vec![CategoriesStateItem::default()]);
        let category = StatefulList::with_items(vec![BoardStateItem::default()]);
        let board = StatefulList::with_items(vec![ThreadStateItem::default()]);
        let thread = StatefulList::with_items(vec![ThreadPostStateItem::default()]);
        let home = StatefulList::with_items(vec![
            HomeStateItem::new(HomeItem::Bookmark),
            HomeStateItem::new(HomeItem::AllChannels),
            HomeStateItem::new(HomeItem::Settings),
        ]);
        let bookmark = StatefulList::with_items(vec![BookmarkStateItem::default()]);

        App {
            left_tabs,
            right_tabs,
            layout,
            message: "".to_string(),
            theme: Theme::default(),
            home,
            bookmark,
            bbsmenu,
            categories,
            category,
            board,
            thread,
        }
    }
}

// GET
impl App {
    pub fn get_menu_id(&mut self) -> i32 {
        self.bbsmenu.items[self.bbsmenu.state.selected().unwrap_or(0)].id
    }
    pub fn get_categories(&self) -> i32 {
        self.categories.items[self.categories.state.selected().unwrap_or(0)].id
    }
    pub fn get_category_name(&self) -> String {
        self.categories.items[self.categories.state.selected().unwrap_or(0)]
            .name
            .clone()
    }
    pub fn get_category_id(&self) -> i32 {
        self.categories.items[self.categories.state.selected().unwrap_or(0)].id
    }
    pub fn get_board_id(&self) -> i32 {
        self.category.items[self.category.state.selected().unwrap_or(0)].id
    }
    pub fn get_thread_id(&self) -> i32 {
        self.board.items[self.board.state.selected().unwrap_or(0)].id
    }
    pub fn get_thread_post_id(&self) -> i32 {
        self.thread.items[self.thread.state.selected().unwrap_or(0)].id
    }
}

impl App {
    pub async fn update(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Get => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Home => {}
                            LeftTabItem::Bookmarks => self.update_bookmark().await?,
                            LeftTabItem::Bbsmenu => self.update_bbsmenu().await?,
                            LeftTabItem::Categories => self.update_categories().await?,
                            LeftTabItem::Category(..) => self.update_category().await?,
                            LeftTabItem::Board(..) => self.update_board().await?,
                            LeftTabItem::Settings => (),
                        }
                    }
                    Pane::Main => {
                        match self.right_tabs.get() {
                            RightTabItem::Thread(..) => self.update_thread().await?,
                        }
                    }
                    _ => (),
                }
                Ok(())
            }
            Event::Down => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Home => self.home.next(),
                            LeftTabItem::Bookmarks => self.bookmark.next(),
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
                    _ => {}
                }
                Ok(())
            }
            Event::Up => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Home => self.home.prev(),
                            LeftTabItem::Bookmarks => self.bookmark.prev(),
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
                    _ => {}
                }
                Ok(())
            }
            Event::ScrollToTop => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Home => self.home.state.select(Some(0)),
                            LeftTabItem::Bookmarks => self.bookmark.state.select(Some(0)),
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
                    _ => {}
                }
                Ok(())
            }
            Event::ScrollToBottom => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Home => {
                                self.home.state.select(Some(self.home.items.len() - 1))
                            }
                            LeftTabItem::Bookmarks => {
                                self.bookmark
                                    .state
                                    .select(Some(self.bookmark.items.len() - 1))
                            }
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
                    _ => {}
                }
                Ok(())
            }
            Event::RemoveHistory => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Home => {}
                            _ => self.left_tabs.hidtory_remove(),
                        }
                        Ok(())
                    }
                    Pane::Main => {
                        match self.right_tabs.get() {
                            RightTabItem::Thread(..) => {
                                self.layout.toggle_focus_pane();
                            }
                        }
                        Ok(())
                    }
                    _ => Ok(()),
                }
            }
            Event::Right => {
                if self.layout.focus_pane == Pane::Main {
                    let menu_id = self.bbsmenu.items[self.bbsmenu.state.selected().unwrap_or(0)].id;
                    let category_id =
                        self.categories.items[self.categories.state.selected().unwrap_or(0)].id;
                    self.right_tabs.next();
                }
                Ok(())
            }
            Event::Left => {
                if self.layout.focus_pane == Pane::Main {
                    self.update_thread().await?;
                    self.right_tabs.previous();
                }
                Ok(())
            }
            Event::Tab => Ok(()),
            Event::Enter => {
                match self.layout.focus_pane {
                    Pane::Side => {
                        match self.left_tabs.get() {
                            LeftTabItem::Home => {
                                let home_item = self.home.items[self.home.selected()].clone().item;
                                match home_item {
                                    HomeItem::Bookmark => {
                                        self.update_bookmark().await?;
                                        self.layout.focus_pane = Pane::Side;
                                        self.left_tabs.history_add(LeftTabItem::Bookmarks);
                                        self.left_tabs.next();
                                    }
                                    HomeItem::Settings => {
                                        self.layout.focus_pane = Pane::Side;
                                        self.left_tabs.history_add(LeftTabItem::Settings);
                                        self.left_tabs.next();
                                    }
                                    HomeItem::AllChannels => {
                                        self.layout.focus_pane = Pane::Side;
                                        self.left_tabs.history_add(LeftTabItem::Bbsmenu);
                                        self.left_tabs.next();
                                    }
                                }
                            }
                            LeftTabItem::Bookmarks => {
                                self.layout.focus_pane = Pane::Side;
                                // self.update_bookmark().await?;
                                self.layout.focus_pane = Pane::Main;
                            }
                            LeftTabItem::Bbsmenu => {
                                self.layout.focus_pane = Pane::Side;
                                let _ = self.update_categories().await?;
                                self.left_tabs.history_add(LeftTabItem::Categories);
                                self.left_tabs.next();
                            }
                            LeftTabItem::Categories => {
                                self.layout.focus_pane = Pane::Side;
                                let _ = self.update_category().await;

                                self.left_tabs
                                    .history_add(LeftTabItem::Category(self.get_category_name()));
                                self.left_tabs.next();
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
                                    .history_add(LeftTabItem::Board(categ.name.clone()));
                                self.left_tabs.next();
                            }
                            LeftTabItem::Board(..) => {
                                let _ = self.update_thread().await;

                                let board =
                                    self.board.items[self.board.state.selected().unwrap()].clone();

                                self.right_tabs
                                    .history_add(RightTabItem::Thread(board.name, board.url));

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
                    _ => Ok(()),
                }
            }
            Event::Message(message) => {
                self.message = message.clone();
                Ok(())
            }
            Event::ToggleBookmark => {
                match self.left_tabs.get() {
                    LeftTabItem::Category(..) => {
                        let board = self.category.items[self.category.selected()].clone();
                        let res = BookmarkStateItem::add(board.url).await;
                        match res {
                            Ok(()) => {
                                self.update_message(format!(
                                    "ブックマークに追加しました。: {}",
                                    board.name
                                ));
                            }
                            Err(e) => {
                                self.update_message(format!(
                                    "ブックマークへの追加に失敗しました。: {}",
                                    board.name
                                ));
                            }
                        }
                    }
                    LeftTabItem::Bookmarks => {
                        let bookmark = self.bookmark.items[self.bookmark.selected()].clone();
                        let res = BookmarkStateItem::delete(bookmark.id).await;
                        match res {
                            Ok(()) => {
                                self.update_message(format!(
                                    "ブックマークから削除しました。: {}",
                                    bookmark.name
                                ));
                            }
                            Err(e) => {
                                self.update_message(format!(
                                    "ブックマークの削除に失敗しました。: {}",
                                    bookmark.name
                                ));
                            }
                        }
                    }
                    _ => {}
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn update_message(&mut self, message: String) {
        self.message = message;
    }

    pub async fn update_bookmark(&mut self) -> Result<()> {
        let bookmarks = BookmarkStateItem::get_all().await?;
        self.bookmark.items = bookmarks;
        Ok(())
    }

    pub async fn update_bbsmenu(&mut self) -> Result<()> {
        self.bbsmenu.items = BbsMenuStateItem::get().await?;
        Ok(())
    }

    pub async fn update_categories(&mut self) -> Result<()> {
        self.bbsmenu.items[self.bbsmenu.selected()].update().await?;

        let menu_id = self.get_menu_id();
        let categories = CategoriesStateItem::get_by_menu_id(menu_id).await?;
        self.categories.items = categories;
        Ok(())
    }

    pub async fn update_category(&mut self) -> Result<()> {
        //現在洗濯中のカテゴりID
        let category_id = self.get_category_id();
        // カテゴリ内の板一覧
        let category = BoardStateItem::get_by_category_id(category_id).await?;
        self.category.items = category;
        Ok(())
    }

    pub async fn update_board(&mut self) -> Result<()> {
        self.category.items[self.category.selected()]
            .clone()
            .fetch()
            .await?;
        let board_id = self.get_board_id();
        self.board.items = ThreadStateItem::get_by_board_id(board_id).await?;
        Ok(())
    }

    pub async fn update_thread(&mut self) -> Result<()> {
        self.board.items[self.board.selected()]
            .clone()
            .fetch()
            .await?;

        let thread_id = self.get_thread_id();
        self.thread.items = ThreadPostStateItem::get_by_thread_id(thread_id).await?;
        Ok(())
    }
}
