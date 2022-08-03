use std::fmt::Display;

use entity::{board, category, menu, prelude::*};
use eyre::{bail, Error, Result};
use migration::async_trait::{self, async_trait};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::database::connect::establish_connection;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct HomeStateItem {
    pub item: HomeItem,
}

impl HomeStateItem {
    pub fn new(item: HomeItem) -> Self {
        HomeStateItem { item }
    }

    pub fn init_vec() -> Vec<HomeStateItem> {
        vec![
            HomeStateItem::new(HomeItem::Bookmark),
            HomeStateItem::new(HomeItem::AllChannels),
            HomeStateItem::new(HomeItem::Settings),
        ]
    }
}
