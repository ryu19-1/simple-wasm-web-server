# simple-wasm-web-server

WebAssembly (WASI) を使った HTTP サーバーの学習プロジェクト。WASIp2 の低レベル API から Spin Framework + WASIp3 の async/await まで、段階的に試している。

## サブプロジェクト

### wasm-http-server

WASIp2 の `wasi:http/incoming-handler` インターフェースを直接実装した最小の HTTP サーバー。

- `wit-bindgen` でバインディングを生成
- `blocking_write_and_flush` による同期 I/O でレスポンスを書き込む
- wasmtime で単体実行可能: `wasmtime serve target/wasm32-wasip2/release/wasm_http_server.wasm`

### auth-middleware

WASIp2 の Component Composition パターンで Authorization ヘッダーを検証するミドルウェア。

- `wasi:http/incoming-handler` を export しつつ import もする（import + export パターン）
- Authorization ヘッダーがなければ 401 を返し、あれば内側のハンドラに委譲
- `wac-cli` で内側のコンポーネントと合成して使う

### hello-p3

Spin Framework + WASIp3 (unstable) で async/await ハンドラを実装した HTTP サーバー。

- `spin_sdk::http_wasip3` による async fn ハンドラ
- auth-middleware → hello-p3 の Service Chaining 構成（`spin.toml` で定義）
- auth-middleware は公開エンドポイント、hello-p3 は private ルートとして Service Chaining 経由のみアクセス可能

## ビルド

各サブプロジェクトは Rust + `wasm32-wasip2` ターゲットでビルドする。

```bash
# wasm-http-server / auth-middleware
cargo build --target wasm32-wasip2 --release

# hello-p3 (Spin)
spin build
spin up
```
