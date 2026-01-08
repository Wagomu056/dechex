# コードスタイルと規約

## 命名規則
- **構造体**: PascalCase（例: `Converter`）
- **関数/メソッド**: snake_case（例: `convert`, `from_str_radix`）
- **変数**: snake_case（例: `converted_string`, `args`）

## コーディング規約
- **Rust Edition**: 2024を使用
- **エラーハンドリング**: `Result`型を使用し、`match`文で適切に処理
- **文字列**: 文字列スライス操作には`&value[2..]`のような配列スライス構文を使用
- **フォーマット**: `format!`マクロを使用（例: `format!("0x{:X}", value)`）

## ドキュメント
- CLI引数の説明には三重スラッシュコメント `///` を使用
- 構造体とその派生には`#[derive(Parser, Debug)]`などのアトリビュートを使用

## テストの書き方
- テストモジュールは`#[cfg(test)]`アトリビュートでマーク
- テスト関数は`#[test]`アトリビュートでマーク
- テスト関数名は`test_`で始まる説明的な名前を使用（例: `test_hex_to_dec`, `test_dec_to_hex`）
- アサーションには`assert_eq!`マクロを使用
