use entity::{
    board,
    category,
    menu,
    prelude::{Board, Category, Menu},
};
use eyre::{bail, Error, Result};
use migration::{
    async_trait::{self, async_trait},
    OnConflict,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, InsertResult, QueryFilter, Set};

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
    pub async fn init(urls: Vec<String>) {
        let db = establish_connection().await.unwrap();
        for url in urls {
            let menu = Menu::find()
                .filter(menu::Column::Url.eq(url.clone()))
                .one(&db)
                .await
                .unwrap_or_default();
            if None == menu {
                let menu = menu::ActiveModel {
                    url: Set(url.clone()),
                    name: Set(url.clone()),
                    ..Default::default()
                };
                Menu::insert(menu)
                    .on_conflict(
                        OnConflict::column(menu::Column::Url)
                            .do_nothing()
                            .to_owned(),
                    )
                    .exec(&db)
                    .await
                    .unwrap();
            }
        }
    }

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

        let menu_id_org = self.id;
        let mut boards = Vec::new();
        for category in res.menu_list.iter() {
            let category_val = category::ActiveModel {
                name: Set(category.category_name.to_string()),
                menu_id: Set(menu_id_org.clone()),
                m_category_name: Set(format!(
                    "{}{}",
                    menu_id_org.clone(),
                    category.category_name.clone()
                )),
                ..Default::default()
            };

            let category_val = Category::insert(category_val)
                .on_conflict(
                    OnConflict::columns(vec![category::Column::MCategoryName])
                        .do_nothing()
                        .to_owned(),
                )
                .exec(&db)
                .await?;
            let category_val = match category_val {
                InsertResult { last_insert_id } => last_insert_id,
            };

            for board in category.category_content.iter() {
                let board_val = board::ActiveModel {
                    name: Set(board.board_name.to_string()),
                    url: Set(board.url.to_string()),
                    category_id: Set(category_val),
                    menu_id: Set(self.id),
                    mc_board_name: Set(format!(
                        "{}{}{}",
                        menu_id_org.clone(),
                        category.category_name.clone(),
                        board.board_name.clone()
                    )),
                    ..Default::default()
                };
                boards.push(board_val);
            }
        }

        Board::insert_many(boards)
            .on_conflict(
                OnConflict::column(board::Column::McBoardName)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(&db)
            .await?;

        Ok(())
    }
}
