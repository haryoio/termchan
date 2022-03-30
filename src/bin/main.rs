#![feature(let_chains)]
extern crate termch;
use termch::{controller::board::Board, sender};
use tokio;

// 板一覧より板をそれぞれのURLに分割して取得する
// const URL: &str = "https://menu.5ch.net/bbsmenu.html";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let board = Board::new("https://mi.5ch.net/news4vip/subback.html").await?;
    let threads = board.parse().await?;
    let first = threads.get(0).unwrap();

    sender::Sender::new(first.get_url().as_str())?
        .send("ASdあｌｓｋｄｆｈｊぁｋｄｊｆｈぁｓｄｋｊｆはｌｄｊｆはｌｄ")
        .await?;
    Ok(())
}
