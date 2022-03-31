#![feature(let_chains)]
extern crate termch;
use termch::{controller::board::Board, login::Login, sender};
use tokio;

// 板一覧より板をそれぞれのURLに分割して取得する
// const URL: &str = "https://menu.5ch.net/bbsmenu.html";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let board = Board::new("https://mi.5ch.net/news4vip/subback.html")?
        .get()
        .await?;
    let threads = board.parse().await?;
    let first = threads.get(0).unwrap();

    Ok(())
}
