use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Words {}

#[allow(dead_code)]
#[derive(Display, Debug)]
pub enum Help {
    #[display(
        fmt = "q: 終了, jk|↓↑: 移動, Enter: 選択, r: リロード, g: 最下部に移動, G: 最上部に移動, Tab: 次のペイン, Shif+Tab: 前のペイン"
    )]
    Home,

    #[display(
        fmt = "q: 終了, jk|↓↑: 移動, Enter: 選択, r: リロード, g: 最下部に移動, G: 最上部に移動, Tab: 次のペイン, Shif+Tab: 前のペイン"
    )]
    Channels,
    Categories,
    Category,
    Board,
    Thread,
    Bookmark,
    Settings,
}
