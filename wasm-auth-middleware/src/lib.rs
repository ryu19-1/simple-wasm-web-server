wit_bindgen::generate!({
    world: "middleware",
    path: "wit",
    generate_all,
});

struct AuthMiddleware;
export!(AuthMiddleware);

impl exports::wasi::http::incoming_handler::Guest for AuthMiddleware {
    fn handle(
        request: exports::wasi::http::incoming_handler::IncomingRequest,
        response_out: exports::wasi::http::incoming_handler::ResponseOutparam,
    ) {
        // Authorizationヘッダをチェック
        // headers は request の子リソースなので、request より先にdropする必要がある
        let has_auth = {
            let headers = request.headers();
            headers.get(&"authorization").first().is_some()
        };

        if !has_auth {
            // トークンなし → 401を返す（内側のハンドラには委譲しない）
            let resp_headers = wasi::http::types::Headers::new();
            resp_headers
                .set(&"content-type", &[b"text/plain; charset=utf-8".to_vec()])
                .unwrap();

            let response = wasi::http::types::OutgoingResponse::new(resp_headers);
            response.set_status_code(401).unwrap();

            let body = response.body().unwrap();
            let stream = body.write().unwrap();
            stream
                .blocking_write_and_flush(b"401 Unauthorized: missing Authorization header\n")
                .unwrap();
            drop(stream);

            wasi::http::types::OutgoingBody::finish(body, None).unwrap();
            wasi::http::types::ResponseOutparam::set(response_out, Ok(response));
            return;
        }

        // トークンあり → 内側のハンドラに委譲
        wasi::http::incoming_handler::handle(request, response_out);
    }
}
