use entity::{board, board_bookmark, category, menu, prelude::*};
use eyre::{bail, Error, Result};
use migration::{
    async_trait::{self, async_trait},
    JoinType,
    OnConflict,
};
use sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    EntityTrait,
    FromQueryResult,
    QueryFilter,
    QuerySelect,
    RelationTrait,
    Set,
};

use crate::database::connect::establish_connection;

#[derive(Debug, Clone)]
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
#[derive(FromQueryResult)]
struct BookmarkQueryResult {
    id:     i32,
    name:   String,
    url:    String,
    domain: String,
}

impl BookmarkStateItem {
    pub async fn get_all() -> Result<Vec<BookmarkStateItem>> {
        let db = establish_connection().await?;
        let bookmarks: Vec<(board_bookmark::Model, Option<board::Model>)> =
            board_bookmark::Entity::find()
                .find_also_related(Board)
                .all(&db)
                .await?;

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
                id: bookmark.id,
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
        // println!("{:?}", board_bookmark);

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

    pub async fn toggle_bookmark_by_board_id(board_id: i32) -> Result<()> {
        let db = establish_connection().await?;
        let board = board_bookmark::Entity::find()
            .filter(board_bookmark::Column::BoardId.eq(board_id))
            .one(&db)
            .await?;

        match board {
            Some(b) => {
                // board_bookmarkテーブルから削除
                BoardBookmark::delete_by_id(b.id).exec(&db).await?;
            }
            None => {
                let new_bookmark = board_bookmark::ActiveModel {
                    board_id: Set(board_id),
                    ..Default::default()
                };
                BoardBookmark::insert(new_bookmark)
                    .on_conflict(
                        OnConflict::column(board_bookmark::Column::BoardId)
                            .do_nothing()
                            .to_owned(),
                    )
                    .exec(&db)
                    .await?;
            }
        }

        Ok(())
    }
}
