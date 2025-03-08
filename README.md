# Wasm Web Server

WebAssembly (Wasm) で実装されたブラウザ上で動作するWebサーバー。

## 概要

このプロジェクトは、WebAssemblyを使用してブラウザ上で動作するWebサーバーを実装したものです。Service Workerを使用してリクエストをインターセプトし、Wasmで実装されたサーバーロジックで処理を行います。

## 環境準備
- Rust
    - ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
    - ```source "$HOME/.cargo/env"```
- wasm-pack
    - ```cargo install wasm-pack```

## Rustライブラリをインストール
```cargo build```

## Wasmビルド
```wasm-pack build --target web```

### `wasm-pack build`の処理内容

`wasm-pack build`コマンドは以下の処理を順番に実行します：

1. **依存関係の解決とダウンロード**
   - `Cargo.toml`に記載された依存クレートの解決
   - 必要なクレートのダウンロード
   - 通常の`cargo build`と同等の依存関係管理

2. **Rustコードのコンパイル**
   - RustコードをWebAssemblyターゲットに向けてコンパイル
   - コンパイル時の最適化の適用

3. **wasm-bindgenによる処理**
   - RustとJavaScript間の橋渡しとなるグルーコードの生成
   - JavaScriptからRustの関数を呼び出すためのラッパーの生成
   - 必要に応じてTypeScript型定義ファイルの生成

4. **wasm-optによる最適化**
   - WebAssemblyバイナリの最適化
   - バイナリサイズの削減
   - 実行速度の改善

生成されたファイルは`pkg`ディレクトリに出力され、以下のものが含まれます：
- `.wasm`ファイル: コンパイルされたWebAssemblyバイナリ
- `.js`ファイル: JavaScriptグルーコード
- `.d.ts`ファイル: TypeScript型定義（必要な場合）
- `package.json`: npmパッケージ設定

## サーバー起動
```npx live-server```
