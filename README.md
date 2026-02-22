# atcoder

[AtCoder](https://atcoder.jp/) の勉強記録用リポジトリです。

## 技術スタック

| 項目                | 詳細                                                                                |
| ------------------- | ----------------------------------------------------------------------------------- |
| 言語                | Rust (メイン), Python, Shell                                                        |
| Rust ツールチェイン | 1.89.0 (`rustfmt`, `clippy` 含む)                                                   |
| コンテスト管理      | [cargo-compete](https://github.com/qryxip/cargo-compete)                            |
| 主要クレート        | `proconio`, `ac-library-rs`, `itertools`, `num`, `superslice`, `rustc-hash`, `rand` |

## ディレクトリ構成

```
.
├── abc/                # AtCoder Beginner Contest
│   ├── 001-100/
│   ├── 101-200/
│   ├── 201-300/
│   ├── 301-400/
│   └── 401-500/
├── arc/                # AtCoder Regular Contest
├── adt/                # AtCoder Daily Training
├── awc/                # AtCoder Weekday Contest
├── dp/                 # Educational DP Contest
├── tdpc/               # Typical DP Contest
├── typical90/          # 競プロ典型 90 問
├── tessoku-book/       # 競技プログラミングの鉄則 演習問題
├── joi/                # JOI
├── practice2/          # AtCoder Library Practice Contest
├── lib/                # 自作ライブラリ的なもの (コピペして使う)
├── scripts/            # 補助スクリプト
├── memo.md             # 勉強したことのメモ
├── Manhattan distance memo.md  # マンハッタン距離の O(N) 解法メモ
├── compete.toml        # cargo-compete 設定
├── rust-toolchain.toml # Rust ツールチェイン設定
└── rustfmt.toml        # rustfmt 設定
```

## セットアップ

### 前提

- [Rust](https://www.rust-lang.org/tools/install) (1.89.0)
- [cargo-compete](https://github.com/qryxip/cargo-compete)

### 新しいコンテストを始める

```bash
# コンテストのパッケージを作成 (例: abc400), change directory, open a file with VS Code and initial bild
target=abc400 && cargo compete n $target && cd $target && code src/bin/a.rs && cargo compete t a

```

### テスト & 提出

```bash
# テストケースを実行
cargo compete t a

# 提出
cargo compete s a
```

## テンプレート

`cargo compete new` で生成されるソースコードのテンプレート (compete.toml で定義)

```rust
#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::*};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::*;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let ans = 0;
    println!("{}", ans);
}
```

## リンク

- [AtCoder](https://atcoder.jp/)
- [cargo-compete](https://github.com/qryxip/cargo-compete)
- [ac-library-rs](https://github.com/rust-lang-ja/ac-library-rs)
