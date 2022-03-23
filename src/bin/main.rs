#![feature(let_chains)]
extern crate SCHCLIENT;
use tokio;
use SCHCLIENT::thread::ThreadList;

// 板一覧より板をそれぞれのURLに分割して取得する
// const URL: &str = "https://menu.5ch.net/bbsmenu.html";
#[tokio::main]
async fn main() {
    let thread_reader = ThreadList::new("https://mi.5ch.net/news4vip/subback.html");
    let thread_list = thread_reader.fetch().await;
    let thread_list = &*thread_list.threads.borrow_mut();
    // for thread in thread_list {
    //     println!(
    //         "{} {} {} {}",
    //         thread.board_key, thread.index, thread.title, thread.id
    //     );
    // }
    let thread = thread_list.get(0).unwrap();
    println!("{:?}", thread);
    let newest = thread.newest_url();
    let all = thread.all_url();
    println!("{} {}", newest, all);

    let fetched = thread.fetch().await;
}
