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

## サーバー起動
```npx live-server```
