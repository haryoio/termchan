use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HomeItem {
    Bookmark,
    Settings,
    AllChannels,
}

impl Display for HomeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HomeItem::Bookmark => write!(f, "Bookmark"),
            HomeItem::Settings => write!(f, "Settings"),
            HomeItem::AllChannels => write!(f, "All Channels"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeStateItem {
    pub item: HomeItem,
}

impl HomeStateItem {
    pub fn new(item: HomeItem) -> Self {
        HomeStateItem { item }
    }
}
