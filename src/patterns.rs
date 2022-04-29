use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// スレッドをパースし、リプライ一覧を取得する。
    ///
    /// 出力されるキャプチャの形式は以下の通り。
    ///
    /// |   | name     | description |
    /// |---|----------|-------------|
    /// | 1 | reply_id | リプライID  |
    /// | 2 | name     | コテ        |
    /// | 3 | date     | 投稿日時    |
    /// | 4 | id       | ユーザID    |
    /// | 5 | message  | 本文        |
    pub static ref REPLIES_RE: Regex = Regex::new(r###"<div class="meta"><span class="number">(?P<reply_id>\d+)</span><span class="name"><b>(?P<name>.*?)</b></span><span class="date">(?P<date>.*?)</span><span class="uid">(?P<id>.*?)</span></div><div class="message"><span class="escaped">(?P<message>.*?)</span></div>"###).unwrap();

    pub static ref REPLIES_SC_RE: Regex = Regex::new(r###"<dt>(?P<reply_id>\d+) ：<font.*><b>(?P<name>.*)<[/]b><[/]font>：(?P<date>.*) (?P<id>.*)<dd>(?P<message>.*)"###).unwrap();

    /// スレッド一覧ページの文字列をパースする。
    ///
    /// 出力されるキャプチャの形式は以下のとおりである
    ///
    /// |   | name  | description          |
    /// |---|-------|----------------------|
    /// | 1 | id    | スレッドID           |
    /// | 2 | index | スレッドインデックス |
    /// | 3 | title | タイトル             |
    /// | 4 | count | レス数               |
    static ref THREAD_LIST_RE: Regex = Regex::new(
            r###"<a href="(?P<id>.*?)/l50">(?P<index>\d+): (?P<title>.*?) \((?P<count>\d+)\)</a>"###
        )
        .expect("failed to create regex");

    pub static ref STOPDONE_RE: Regex = Regex::new(r###"■ このスレッドは過去ログ倉庫に格納されています"###)
            .expect("failed to create regex");

    pub static ref ROADING_RE: Regex = Regex::new(r###"読み込み中。。。"###)
            .expect("failed to create regex");
}

pub fn parse_thread_list(before: &str) -> regex::CaptureMatches {
    THREAD_LIST_RE.captures_iter(&before)
}

pub fn get_error_message(html: &str) -> Option<String> {
    for l in html.lines() {
        if l.contains("ERROR:") {
            return Some(
                l.split("<b>")
                    .nth(1)
                    .unwrap()
                    .split("</b>")
                    .nth(0)
                    .unwrap()
                    .to_string(),
            );
        }
    }
    Some("".to_string())
}
