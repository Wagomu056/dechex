# 推奨コマンド

## ビルドと実行
```bash
# プロジェクトをビルド
cargo build

# リリースビルド
cargo build --release

# アプリケーションを実行（10進数から16進数へ）
cargo run -- 255

# アプリケーションを実行（16進数から10進数へ）
cargo run -- 0xFF

# ヘルプを表示
cargo run -- --help
```

## テスト
```bash
# すべてのテストを実行
cargo test

# 詳細出力でテストを実行
cargo test -- --nocapture

# 特定のテストを実行
cargo test test_hex_to_dec
```

## コード品質
```bash
# コードのフォーマット確認
cargo fmt -- --check

# コードをフォーマット
cargo fmt

# リンターを実行（警告を確認）
cargo clippy

# より厳格なリンターチェック
cargo clippy -- -D warnings
```

## その他の便利なコマンド
```bash
# 依存関係を更新
cargo update

# 未使用の依存関係をチェック
cargo tree

# プロジェクトをクリーン
cargo clean

# ドキュメントを生成して開く
cargo doc --open
```

## macOS固有のユーティリティコマンド
- `ls` - ファイル一覧表示
- `grep` - テキスト検索
- `find` - ファイル検索
- `git` - バージョン管理
- `pbcopy` - クリップボードにコピー
- `pbpaste` - クリップボードから貼り付け
