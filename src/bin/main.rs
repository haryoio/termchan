#![feature(let_chains)]
extern crate SCHCLIENT;
use anyhow::Context;
use tokio;
use SCHCLIENT::controller::board::Board;

// 板一覧より板をそれぞれのURLに分割して取得する
// const URL: &str = "https://menu.5ch.net/bbsmenu.html";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let board = Board::new("https://mi.5ch.net/news4vip/subback.html").await?;
    let threads = board.parse().await;
    for thread in &threads {
        println!("{} {}", thread.get_id(), thread.get_title());
    }
    let first = threads.get(0).unwrap();
    println!("{:?}", first);
    let first_replies = first.get_replies().await?;
    println!("{:?}", first_replies);
    for reply in first_replies {
        println!("{}", reply);
    }
    Ok(())
}
