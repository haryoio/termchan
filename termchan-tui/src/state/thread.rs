use entity::{board, category, menu, prelude::*, thread, thread_post};
use eyre::{bail, Error, Result};
use migration::{
    async_trait::{self, async_trait},
    OnConflict,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use termchan_core::get::thread::Thread;

use crate::database::connect::establish_connection;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ThreadStateItem {
    pub id:          i32,
    pub url:         String,
    pub name:        String,
    pub count:       i32,
    pub ikioi:       f64,
    pub updated_at:  String,
    pub is_read:     bool,
    pub stopdone:    bool,
    pub before_read: i32,
}

impl Default for ThreadStateItem {
    fn default() -> Self {
        ThreadStateItem {
            id:          0,
            url:         String::new(),
            name:        String::new(),
            count:       0,
            ikioi:       0.0,
            updated_at:  String::new(),
            is_read:     false,
            stopdone:    false,
            before_read: 0,
        }
    }
}

impl ThreadStateItem {
    pub async fn get_by_board_id(board_id: i32) -> Result<Vec<ThreadStateItem>> {
        let db = establish_connection().await?;
        let threads = thread::Entity::find()
            .filter(thread::Column::BoardId.eq(board_id))
            .all(&db)
            .await?;
        let mut thread_state_item = Vec::new();
        for thread in threads {
            thread_state_item.push(ThreadStateItem {
                id:          thread.id,
                url:         thread.url.to_string(),
                name:        thread.name.to_string(),
                count:       thread.count,
                ikioi:       thread.ikioi.unwrap_or(0.0),
                updated_at:  thread.updated_at.unwrap_or_default(),
                is_read:     thread.is_read,
                stopdone:    thread.stopdone,
                before_read: thread.before_read,
            });
        }
        Ok(thread_state_item)
    }

    pub async fn fetch(&self) -> Result<()> {
        let db = establish_connection().await?;
        let res = Thread::new(self.url.to_string())?.get().await?;
        let mut new_posts = vec![];
        for item in res.posts {
            new_posts.push(thread_post::ActiveModel {
                thread_id: Set(self.id),
                name: Set(item.name.to_string()),
                index: Set(item.index.try_into().unwrap()),
                post_id: Set(item.post_id),
                message: Set(item.message.json_string()),
                date: Set(Some(item.date.to_string())),
                email: Set(item.email.to_string()),
                thread_id_index: Set(format!("{}_{}", self.id, item.index)),
                ..Default::default()
            });
        }

        let res = ThreadPost::insert_many(new_posts)
            .on_conflict(
                OnConflict::column(thread_post::Column::ThreadIdIndex)
                    // 被アンカー数が変わるので、Messageを更新する必要がある。
                    .update_columns(vec![thread_post::Column::Message])
                    .to_owned(),
            )
            .exec(&db)
            .await?;
        Ok(())
    }
}
