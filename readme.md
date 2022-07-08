# termchan

---

2ch互換掲示板を閲覧できるCLIクライアント及びライブラリ

## インストール

## Examples

リクエストや情報のパースなどを行うapiモジュールと、その情報を見やすいようにするtuiモジュールで構成されます。

- 読み込み

  - bbsmenu

    ```rs
    use termchan::api::access::get::bbsmenu;

    let url = "https://menu.2ch.sc/bbsmenu.html";
    let content = bbsmenu::Bbsmenu::new(url).load().await;
    ```

  - 板一覧
  - スレッド一覧
  - スレッド
- 書き込み
  - スレッド作成
  - レスポンス作成

## インストール

##
