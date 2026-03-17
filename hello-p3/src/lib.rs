use spin_sdk::http_wasip3::{IntoResponse, Request, http_service};

// ★ P3のポイント：async fn でHTTPハンドラが書ける！
// P2ではpollable + blocking_write_and_flush だったのが、
// 普通のRustの型とasync/awaitになった。
#[http_service]
async fn handle_request(req: Request) -> impl IntoResponse {
    let path = req.uri().path().to_string();

    format!("Hello from WASIp3!\nPath: {}\n", path)
}
