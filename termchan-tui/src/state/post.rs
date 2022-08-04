use entity::thread_post;
use eyre::Result;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use termchan_core::get::message::Message;

use crate::database::connect::establish_connection;

#[derive(Debug, Clone)]
pub struct ThreadPostStateItem {
    pub id:      i32,
    pub index:   i32,
    pub post_id: String,
    pub message: Message,
    pub date:    i64,
    pub email:   Option<String>,
    pub name:    String,
}

impl Default for ThreadPostStateItem {
    fn default() -> Self {
        ThreadPostStateItem {
            id:      0,
            index:   0,
            post_id: String::new(),
            message: Message::default(),
            date:    0,
            email:   None,
            name:    String::new(),
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
            let date = post
                .date
                .unwrap_or("0".to_string())
                .parse::<i64>()
                .unwrap_or(0);
            thread_post_state_item.push(ThreadPostStateItem {
                id: post.id,
                index: post.index,
                post_id: post.post_id.to_string(),
                message,
                date,
                email: Some(post.email),
                name: post.name.to_string(),
            });
        }
        Ok(thread_post_state_item)
    }
}
