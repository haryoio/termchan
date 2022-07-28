use entity::{
    board,
    category,
    menu,
    prelude::{Board, Menu},
};
use eyre::{bail, Error, Result};
use migration::async_trait::{self, async_trait};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::database::connect::{establish_connection, Repository};

#[derive(Debug, Clone)]
pub struct BbsMenuStateItem {
    pub id:  i32,
    pub url: String,
}

impl Default for BbsMenuStateItem {
    fn default() -> Self {
        BbsMenuStateItem {
            id:  0,
            url: String::new(),
        }
    }
}

impl BbsMenuStateItem {
    pub async fn get() -> Result<Vec<BbsMenuStateItem>> {
        let db = establish_connection().await?;
        let menus = menu::Entity::find().all(&db).await?;
        let mut bbs_menu_state_item = Vec::new();
        for menu in menus {
            bbs_menu_state_item.push(BbsMenuStateItem {
                id:  menu.id,
                url: menu.url.to_string(),
            });
        }
        Ok(bbs_menu_state_item)
    }
    pub async fn insert(&self, url: &str) -> Result<()> {
        let conn = establish_connection().await?;
        let new_menu = menu::ActiveModel {
            name: Set(url.to_string().to_string()),
            url: Set(url.to_string().to_string()),
            ..Default::default()
        };
        let _ = new_menu.save(&conn).await;
        Ok(())
    }

    pub async fn update(&self) -> Result<()> {
        let db = establish_connection().await?;
        let res = termchan::get::bbsmenu::Bbsmenu::new(self.url.to_string())?
            .get()
            .await?;

        let menu_is_exists = Menu::find()
            .filter(menu::Column::Url.contains(&self.url))
            .one(&db)
            .await?;

        if menu_is_exists.is_some() {
            return Ok(());
        }

        let menu_id_org = match menu_is_exists {
            Some(menu) => menu.id,
            None => {
                menu::ActiveModel {
                    name: Set(self.url.to_string()),
                    url: Set(self.url.to_string()),
                    ..Default::default()
                }
                .save(&db)
                .await?
                .id
                .unwrap()
            }
        };

        let mut boards = Vec::new();
        for category in res.menu_list.iter() {
            let category_val = category::ActiveModel {
                name: Set(category.category_name.to_string()),
                menu_id: Set(menu_id_org.clone()),
                ..Default::default()
            }
            .save(&db)
            .await?;

            for board in category.category_content.iter() {
                let board_val = board::ActiveModel {
                    name: Set(board.board_name.to_string()),
                    url: Set(board.url.to_string()),
                    category_id: Set(menu_id_org.clone()),
                    menu_id: Set(category_val.clone().id.unwrap()),
                    ..Default::default()
                };
                boards.push(board_val);
            }
        }

        let _ = Board::insert_many(boards).exec(&db).await?;

        Ok(())
    }
}
