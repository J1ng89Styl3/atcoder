## 競プロをちゃんとやろう

https://kenkoooo.com/atcoder/#/table/J1ng89Styl3

## 使い方

cargo compete new {コンテスト名}

cd {コンテスト名}

cargo compete test {問題名}

cargo compete submit {問題名}

cargo run --bin abcxxx-a

## cargo-compete にログインできない場合

Cloudflare Turnstile の影響で `cargo compete login atcoder` が失敗する場合は、ブラウザで AtCoder にログインして `REVEL_SESSION` を取得し、cargo-compete の `cookies.jsonl` に反映する。

参考: https://zenn.dev/stddev/articles/388b4d2acb248e

このリポジトリでは次のスクリプトで反映できる。

```sh
ruby scripts/update-cargo-compete-cookie.rb
```

実行後、ブラウザの DevTools で確認した `REVEL_SESSION` の値を貼り付ける。`REVEL_SESSION=...` 形式で貼り付けても、値だけを貼り付けてもよい。

macOS では通常 `~/Library/Application Support/cargo-compete/cookies.jsonl` を更新する。Linux などでは `$XDG_DATA_HOME/cargo-compete/cookies.jsonl` または `~/.local/share/cargo-compete/cookies.jsonl` を使う。別の場所を更新したい場合は `CARGO_COMPETE_COOKIES` で指定できる。
