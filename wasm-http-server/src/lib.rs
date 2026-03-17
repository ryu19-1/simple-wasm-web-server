wit_bindgen::generate!({
    world: "server",
    path: "wit",
    generate_all,
});

struct MyServer;

export!(MyServer);

impl exports::wasi::http::incoming_handler::Guest for MyServer {
    fn handle(
        request: exports::wasi::http::incoming_handler::IncomingRequest,
        response_out: exports::wasi::http::incoming_handler::ResponseOutparam,
    ) {
        let path = request.path_with_query().unwrap_or_else(|| "/".to_string());

        let body = format!(
            "Hello from WebAssembly!\nPath: {}\nPowered by WASI HTTP\n",
            path
        );

        let headers = wasi::http::types::Headers::new();
        headers
            .set(&"content-type", &[b"text/plain; charset=utf-8".to_vec()])
            .unwrap();

        let response = wasi::http::types::OutgoingResponse::new(headers);
        response.set_status_code(200).unwrap();

        let out_body = response.body().unwrap();
        let out_stream = out_body.write().unwrap();
        out_stream
            .blocking_write_and_flush(body.as_bytes())
            .unwrap();
        drop(out_stream);

        wasi::http::types::OutgoingBody::finish(out_body, None).unwrap();
        wasi::http::types::ResponseOutparam::set(response_out, Ok(response));
    }
}
