use entity::{board, category, menu, prelude::*, thread};
use eyre::{bail, Error, Result};
use migration::{
    async_trait::{self, async_trait},
    OnConflict,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::database::connect::establish_connection;

#[derive(Debug, Clone)]
pub struct BoardStateItem {
    pub id:   i32,
    pub url:  String,
    pub name: String,
}

impl Default for BoardStateItem {
    fn default() -> Self {
        BoardStateItem {
            id:   0,
            url:  String::new(),
            name: String::new(),
        }
    }
}

impl BoardStateItem {
    pub async fn get_by_category_id(category_id: i32) -> Result<Vec<BoardStateItem>> {
        let db = establish_connection().await?;
        let boards = board::Entity::find()
            .filter(board::Column::CategoryId.eq(category_id))
            .all(&db)
            .await?;
        let mut board_state_item = Vec::new();
        for board in boards {
            board_state_item.push(BoardStateItem {
                id:   board.id,
                url:  board.url.to_string(),
                name: board.name.to_string(),
            });
        }
        Ok(board_state_item)
    }
    /// 板URLからスレッド一覧を取得する。
    pub async fn fetch(&self) -> Result<()> {
        let db = establish_connection().await?;
        let res = termchan::get::board::Board::new(self.url.to_string())?
            .get()
            .await?;
        let mut new_threads = vec![];
        for item in res {
            new_threads.push(thread::ActiveModel {
                name: Set(item.name.to_string()),
                url: Set(item.url.to_string()),
                count: Set(item.count),
                ikioi: Set(Some(item.ikioi)),
                updated_at: Set(Some(item.created_at.to_string())),
                board_id: Set(self.id),
                ..Default::default()
            });
        }
        let res = Thread::insert_many(new_threads)
            .on_conflict(
                OnConflict::column(thread::Column::Url)
                    .update_column(thread::Column::Count)
                    .to_owned(),
            )
            .exec(&db)
            .await?;
        Ok(())
    }
}
