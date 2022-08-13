use entity::{board, board_bookmark, prelude::*, thread};
use eyre::{bail, Result};
use futures::future::join_all;
use migration::{Condition, Expr, OnConflict, Query, Value};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use termchan_core::get::board::Board;

use crate::database::connect::establish_connection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkStateItem {
    pub id:     i32,
    pub name:   String,
    pub url:    String,
    pub domain: String,
}

impl Default for BookmarkStateItem {
    fn default() -> Self {
        BookmarkStateItem {
            id:     0,
            name:   String::new(),
            url:    String::new(),
            domain: String::new(),
        }
    }
}

impl BookmarkStateItem {
    pub async fn get_all() -> Result<Vec<BookmarkStateItem>> {
        let db = establish_connection().await?;
        let bookmarks: Vec<(board_bookmark::Model, Option<board::Model>)> =
            board_bookmark::Entity::find()
                .find_also_related(Board)
                .all(&db)
                .await?;

        if bookmarks.len() == 0 {
            bail!("No bookmark found.");
        }

        let mut bookmarks_state_item = Vec::new();
        for bookmark in bookmarks {
            let board = bookmark.1.unwrap().clone();
            let bookmark = bookmark.0;
            let url = board.url.to_string();
            let domain = reqwest::Url::parse(&url)
                .unwrap()
                .domain()
                .unwrap()
                .to_string();
            bookmarks_state_item.push(BookmarkStateItem {
                id: bookmark.board_id,
                name: board.name.to_string(),
                url,
                domain,
            });
        }
        Ok(bookmarks_state_item)
    }

    pub async fn add(url: String) -> Result<()> {
        let db = establish_connection().await?;

        let board = board::Entity::find()
            .filter(board::Column::Url.eq(url.clone()))
            .one(&db)
            .await?;

        let board = match board {
            Some(board) => board,
            None => bail!("board not found {}", url),
        };

        let board_bookmark = board_bookmark::ActiveModel {
            board_id: Set(board.id),
            rating: Set(0),
            ..Default::default()
        };

        BoardBookmark::insert(board_bookmark)
            .on_conflict(
                OnConflict::column(board_bookmark::Column::BoardId)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(&db)
            .await?;
        Ok(())
    }

    pub async fn delete(id: i32) -> Result<()> {
        let db = establish_connection().await?;
        BoardBookmark::delete_by_id(id).exec(&db).await.unwrap();
        Ok(())
    }

    /// 板URLからスレッド一覧を取得する。
    pub async fn fetch(&self) -> Result<()> {
        let db = establish_connection().await?;
        info!("fetch board from: {}", self.url.to_string());
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

        let res: Vec<(thread::Model, Option<board::Model>)> = Thread::find()
            .find_also_related(Board)
            .filter(board::Column::Url.eq(self.url.to_string()))
            .all(&db)
            .await?;
        warn!("current thread len {:?}", res.len());
        // update

        let res = Thread::update_many()
            .col_expr(thread::Column::Stopdone, Expr::value(true))
            .filter(
                Condition::any().add(
                    thread::Column::BoardId.in_subquery(
                        Query::select()
                            .column(board::Column::Id)
                            .from(Board)
                            .and_where(Expr::col(board::Column::Url).eq(self.url.to_string()))
                            .to_owned(),
                    ),
                ),
            )
            .exec(&db)
            .await;
        warn!("update thread len {:?}", res);

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
