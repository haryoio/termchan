use lazy_static::lazy_static;
use regex::Regex;

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
///
pub fn parse_replies(before: &str) -> regex::CaptureMatches {
    lazy_static! {
        static ref RE:Regex = Regex::new(r###"<div class="meta"><span class="number">(?P<reply_id>\d+)</span><span class="name"><b>(?P<name>.*?)</b></span><span class="date">(?P<date>.*?)</span><span class="uid">(?P<id>.*?)</span></div><div class="message"><span class="escaped">(?P<message>.*?)</span></div>"###).unwrap();
    }
    RE.captures_iter(&before)
}

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
pub fn parse_thread_list(before: &str) -> regex::CaptureMatches {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r###"<a href="(?P<id>.*?)/l50">(?P<index>\d+): (?P<title>.*?) \((?P<count>\d+)\)</a>"###
        )
        .expect("failed to create regex");
    }
    RE.captures_iter(&before)
}

///
pub fn is_stopdone(before: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r###"■ このスレッドは過去ログ倉庫に格納されています"###)
            .expect("failed to create regex");
    }
    RE.is_match(&before)
}
