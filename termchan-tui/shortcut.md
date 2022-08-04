# ショートカットチートシート

`allow` = `←↑↓→`

## Command Mode

| Command                   | Hotkey1  | Hotkey2 |
| :------------------------ | :------- | :------ |
| Next tab                  | Alt + l  |         |
| Prev tab                  | Alt + h  |         |
| Scroll up                 | k        |         |
| Scroll down               | j        |         |
| Scroll top                | g        |         |
| Scroll bottom             | G        |         |
| Select Item               | Space    |         |
| Toggle Focus Pane         | Tab      |         |
| Toggle Sidebar Visibility | Ctrl + b |         |
| Toggle Bookmark           | Ctrl + d |         |

マイグレーション用バイナリも一緒にzipへ入れる

## GithubActions

### Actionが実行されるタイミングを指定する

`.github/workflows/release.yml`に記述する

起動するタイミングは`on`以下で設定できる。
以下の例は`v*`にマッチするタグがプッシュされたらActionを実行する
```yml
on:
    push:
        tabs: ['v[0-9]+.[0-9]+.[0-9]']
```

### 環境変数を指定する

```yml
env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
    CARGO_TERM_COLOR: always
```

### 走らせるJOBを設定する

```yml
jobs:
    linux:
        runs-on: ubuntsu-18.04

        steps:
            - uses: actions/checkout@v2
            - name: Install dependencies
                run: |
                    sudo apt-get update
                    sudo apt-get install cmake
            - name: Update rust
                run: rustup update
            - name: Build
                run: cargo build




