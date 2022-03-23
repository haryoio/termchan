# SCH - linux 5ch client

- reader
  - 掲示板の構造を抽象化したデータにするモジュール
- writer
  - 掲示板への書き込みを抽象化したモジュール
- access
  - プロキシなど
- pluguin
  - 組み込めるやつ
- cli
  - CLIでの書き込みに対応
- hosh
  - 保守
- GUI
  - GUI

## データ出力

板一覧
スレ一覧
レス一覧

## データ処理

ShiftJISからUTF-8へ変換
UTF-8からShift-JISへの変換
スレ一覧のHTMLからスレ一覧を抽出
スレHTMLからレス一覧を抽出

設定

- tomlによる設定
  - 板一覧URLを設定
  - 浪人ログインなど

TODO
[] いた一覧を取得
[] 板URLからスレッド一覧を取得
[] スレッドURLからスレ一覧を取得
[] スレ一覧からスレURLを取得
[] スレURLからレス一覧を取得

[] レス
[] プロキシを介したアクセス
[]

## 板一覧の取得


## スレッド一覧の取得

```url
https://<サーバ名>/板キー/subback.html
```

```html
<small>
  <div>
    <a href="<thread_id>">[index]: [title]（[count]）</a>
  </div>
</small>
```

## スレッドの取得

```url
https://mi.5ch.net/test/read.cgi/news4vip/1647788488/l50
https://<サーバ名>/test/read.cgi/<板キー>/<スレッドID>
```