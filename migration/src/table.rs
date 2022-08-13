use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Menu {
    Table,
    Id,
    Url,
    Name,
}

#[derive(Iden)]
pub enum Category {
    Table,
    Id,
    Name,
    MCategoryName,
    MenuId,
}

#[derive(Iden)]
pub enum Board {
    Table,
    Id,
    Name,
    Url,
    MCBoardName,
    MenuId,
    CategoryId,
}

#[derive(Iden)]
pub enum BoardBookmark {
    Table,
    Id,
    Rating,
    BoardId,
}

#[derive(Iden)]
pub enum Thread {
    Table,
    Id,
    Index,
    Url,
    Name,
    Count,
    Ikioi,
    CreatedTime,
    /// 読み込んだことがあるか
    IsRead,
    /// Dat落ち
    Stopdone,
    /// ここまで読んだ
    BeforeRead,
    BoardId,
}

#[derive(Iden)]
pub enum ThreadPost {
    Table,
    Id,
    /// ランダムなユーザーID
    /// データベースのリレーションとは関係ない
    PostId,
    /// 投稿番号
    Index,
    /// ユーザネーム
    Name,
    /// sageとかageとか
    Email,
    /// unix time
    Date,
    /// 投稿内容
    Message,
    /// 一位に識別するためのやつ
    ThreadIdIndex,
    /// スレッドID データベースのThreadテーブルに対応
    ThreadId,
}
#[derive(Iden)]
pub enum Image {
    Table,
    Id,
    Url,
    Size,
    SavePath,
}
