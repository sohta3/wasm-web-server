# Wasm Web Server

WebAssemblyで実装されたTODOアプリケーション。ブラウザ上で動作するWebサーバーとして機能し、Service WorkerとCache Storageを活用してデータを永続化します。

## 機能

- TODOの作成、読み取り、更新、削除（CRUD操作）
- ブラウザ上でのデータ永続化（Cache Storage使用）
- Service Workerによるリクエストのインターセプト
- オフライン対応の可能性

## アーキテクチャ

```
[ブラウザ]
    ↓ HTTPリクエスト（localhost:3000/api/todos）
[Service Worker]
    ↓ リクエストのインターセプトとルーティング
[Wasm WebServer]
    ↓ TODOデータの処理
[Cache Storage]
    ↓ データの永続化
```

## 技術スタック

- **WebAssembly**: Rustコードをブラウザで実行可能なバイナリにコンパイル
- **Rust**: サーバーロジックの実装
- **Service Worker**: HTTPリクエストのインターセプトとルーティング
- **Cache Storage**: TODOデータの永続化
- **wasm-bindgen**: RustとJavaScript間のバインディング

## 環境準備

### 必要なツール

1. **Rust**のインストール
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```

2. **wasm-pack**のインストール
   ```bash
   cargo install wasm-pack
   ```

### ビルドと実行

1. **Wasmのビルド**
   ```bash
   wasm-pack build --target web
   ```

2. **開発サーバーの起動**
   ```bash
   npx live-server
   ```

## `wasm-pack build`の処理内容

`wasm-pack build --target web`コマンドは以下の処理を実行します：

1. **依存関係の解決とダウンロード**
   - `Cargo.toml`に記載された依存クレートの解決
   - 必要なクレートのダウンロード
   - `cargo build`相当の依存関係管理

2. **Rustコードのコンパイル**
   - RustコードをWebAssemblyにコンパイル（`wasm32-unknown-unknown`ターゲット）
   - リリースビルドの場合、最適化も適用

3. **wasm-bindgenによる処理**
   - RustとJavaScript間の相互運用コードの生成
   - JavaScriptからRustの関数を呼び出すためのラッパー生成
   - TypeScript型定義ファイルの生成

4. **wasm-optによる最適化**
   - WebAssemblyバイナリの最適化
   - バイナリサイズの削減
   - 実行速度の改善のための最適化

### 生成物（`pkg`ディレクトリ）

- **`.wasm`**: コンパイル済みWebAssemblyバイナリ
- **`.js`**: JavaScriptバインディング（グルーコード）
- **`.d.ts`**: TypeScript型定義
- **`package.json`**: npmパッケージ設定

## APIエンドポイント

- **GET /api/todos**
  - TODOリストの取得
  - レスポンス: `[{"id": number, "title": string, "completed": boolean}, ...]`

- **POST /api/todos**
  - 新規TODOの作成
  - リクエストボディ: `{"title": string, "completed": boolean?}`
  - レスポンス: `{"status": "created"}`

- **PUT /api/todos/:id**
  - TODOの更新
  - リクエストボディ: `{"title": string?, "completed": boolean?}`
  - レスポンス: `{"status": "updated"}` または `{"error": "Todo not found"}`

- **DELETE /api/todos/:id**
  - TODOの削除
  - レスポンス: `{"status": "deleted"}` または `{"error": "Todo not found"}`

## データ永続化

Cache Storage APIを使用してTODOデータを永続化しています：
- ブラウザのCache Storageに保存
- Service Worker経由でアクセス
- JSONフォーマットでデータを保存
- ブラウザを閉じても保持

## 開発者向け情報

### デバッグ方法
1. Chrome DevToolsを開く
2. Applicationタブを選択
3. Service Workersセクションで登録状態を確認
4. Cache Storageセクションでデータを確認
5. Consoleタブでログを確認

### 注意事項
- プライベートブラウジングモードではデータが永続化されない場合があります
- ブラウザのキャッシュクリアでデータが消去される可能性があります
- Service Workerの更新時は再登録が必要な場合があります
