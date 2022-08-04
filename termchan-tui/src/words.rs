use derive_more::{Add, Display};
use serde::{Deserialize, Serialize};
use tui::style::Style;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Words {}

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

pub struct Word {
    pub text:  String,
    pub style: Style,
}
