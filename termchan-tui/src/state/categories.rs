use entity::category;
use eyre::Result;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::database::connect::establish_connection;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoriesStateItem {
    pub id:   i32,
    pub name: String,
}

impl Default for CategoriesStateItem {
    fn default() -> Self {
        CategoriesStateItem {
            id:   0,
            name: String::new(),
        }
    }
}

impl CategoriesStateItem {
    /// 全てのカテゴリ名を取得する。
    pub async fn get_by_menu_id(id: i32) -> Result<Vec<CategoriesStateItem>> {
        let db = establish_connection().await?;
        let categories = category::Entity::find()
            .filter(category::Column::MenuId.eq(id))
            .all(&db)
            .await?;
        let mut categories_state_item = Vec::new();
        for category in categories {
            categories_state_item.push(CategoriesStateItem {
                id:   category.id,
                name: category.name.to_string(),
            });
        }
        Ok(categories_state_item)
    }
}
