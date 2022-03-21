use std::{char, fs, io::Read};
use tokio;

mod util;

// 板一覧より板をそれぞれのURLに分割して取得する
// const URL: &str = "https://menu.5ch.net/bbsmenu.html";
#[tokio::main]
async fn main() {
    let sjis_path = "menu_sjis.html";
    let s = fs::read(sjis_path).unwrap();
    let s = util::sjis_to_utf8(&s);
    println!("{}", s);
}
