use entity::{board, prelude::*, thread};
use eyre::Result;
use migration::OnConflict;
use sea_orm::{
    sea_query::{Expr, Value},
    ColumnTrait,
    EntityTrait,
    QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};
use termchan_core::get::board::Board;

use crate::database::connect::establish_connection;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let res = Board::new(self.url.to_string())?.get().await?;
        let mut new_threads = vec![];
        // 一旦スレッド一覧はi新規スレッドとして取得する。
        for item in res {
            new_threads.push(thread::ActiveModel {
                index: Set(item.index),
                name: Set(item.name.to_string()),
                url: Set(item.url.to_string()),
                count: Set(item.count),
                ikioi: Set(Some(item.ikioi)),
                created_time: Set(Some(item.created_time.timestamp())),
                board_id: Set(self.id),
                stopdone: Set(false),
                is_read: Set(false),
                ..Default::default()
            });
        }
        // DBにあるスレッドはすべてDAT落ち判定にする。
        let res = Thread::update_many()
            .filter(thread::Column::BoardId.eq(self.id))
            .col_expr(
                thread::Column::Stopdone,
                Expr::value(Value::Bool(Some(true))),
            )
            .exec(&db)
            .await?;

        warn!("{:?}", res);
        for thread in &new_threads {
            info!("{:?}", thread);
        }

        // Dat落ちしていないスレッドはDat落ちが解除される。
        let res = Thread::insert_many(new_threads)
            .on_conflict(
                OnConflict::column(thread::Column::Url)
                    .update_columns(vec![
                        thread::Column::Count,
                        thread::Column::Ikioi,
                        thread::Column::Stopdone,
                        thread::Column::Index,
                    ])
                    .to_owned(),
            )
            .exec(&db)
            .await?;
        info!("{:?}", res);
        Ok(())
    }
}
