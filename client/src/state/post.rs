use entity::{board, category, menu, prelude::*, thread, thread_post};
use eyre::{bail, Error, Result};
use migration::{
    async_trait::{self, async_trait},
    OnConflict,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde_json::json;
use termchan::get::message::Message;

use crate::database::connect::establish_connection;

#[derive(Debug, Clone)]
pub struct ThreadPostStateItem {
    pub id:      i32,
    pub index:   i32,
    pub post_id: String,
    pub message: Message,
    pub date:    Option<String>,
    pub email:   Option<String>,
}

impl Default for ThreadPostStateItem {
    fn default() -> Self {
        ThreadPostStateItem {
            id:      0,
            index:   0,
            post_id: String::new(),
            message: Message::default(),
            date:    None,
            email:   None,
        }
    }
}

impl ThreadPostStateItem {
    pub async fn get_by_thread_id(thread_id: i32) -> Result<Vec<ThreadPostStateItem>> {
        let db = establish_connection().await?;
        let posts = thread_post::Entity::find()
            .filter(thread_post::Column::ThreadId.eq(thread_id))
            .all(&db)
            .await?;
        let mut thread_post_state_item = Vec::new();
        for post in posts {
            let message: Message = serde_json::from_str(&post.message)?;
            thread_post_state_item.push(ThreadPostStateItem {
                id: post.id,
                index: post.index,
                post_id: post.post_id.to_string(),
                message,
                date: post.date,
                email: Some(post.email),
            });
        }
        Ok(thread_post_state_item)
    }
}
