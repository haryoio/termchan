#![feature(let_chains)]
extern crate SCHCLIENT;
use tokio;
use SCHCLIENT::models::board::Board;

// 板一覧より板をそれぞれのURLに分割して取得する
// const URL: &str = "https://menu.5ch.net/bbsmenu.html";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let board = Board::new("https://mi.5ch.net/news4vip/subback.html").await;
    let threads = board.parse();
    // for thread in thread_list {
    //     println!(
    //         "{} {} {} {}",
    //         thread.board_key, thread.index, thread.title, thread.id
    //     );
    // }
    let thread = println!("{:?}", thread);
    let newest = thread.get_newest_url();
    let all = thread.get_all_url();
    println!("{} {}", newest, all);

    let fetched = thread.fetch().await;
    Ok(())
}
