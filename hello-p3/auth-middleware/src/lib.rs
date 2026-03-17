use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[http_component]
async fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    // Authorization ヘッダーの検証
    let has_auth = req.header("authorization").is_some();

    if !has_auth {
        return Ok(Response::builder()
            .status(401)
            .header("content-type", "text/plain; charset=utf-8")
            .body("401 Unauthorized: missing Authorization header\n")
            .build());
    }

    // 内部コンポーネントに転送
    let path = req.path().to_string();
    let method = req.method().clone();
    let body = req.into_body();

    let internal_req = Request::builder()
        .method(method)
        .uri(format!("http://hello-p3.spin.internal{}", path))
        .body(body)
        .build();

    let resp: Response = spin_sdk::http::send(internal_req).await?;
    Ok(resp)
}
