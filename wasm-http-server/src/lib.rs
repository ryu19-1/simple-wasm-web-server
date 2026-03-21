use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::{
    Headers, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

wit_bindgen::generate!({
    world: "server",
    path: "wit",
    generate_all,
});

struct MyServer;
export!(MyServer);

impl Guest for MyServer {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let path = request.path_with_query().unwrap_or_else(|| "/".to_string());

        let body = format!(
            "Hello from WebAssembly!\nPath: {}\nPowered by WASI HTTP\n",
            path
        );

        let headers = Headers::new();
        headers
            .set(&"content-type", &[b"text/plain; charset=utf-8".to_vec()])
            .unwrap();

        let response = OutgoingResponse::new(headers);
        response.set_status_code(200).unwrap();

        let out_body = response.body().unwrap();
        let out_stream = out_body.write().unwrap();
        out_stream
            .blocking_write_and_flush(body.as_bytes())
            .unwrap();
        drop(out_stream);

        OutgoingBody::finish(out_body, None).unwrap();
        ResponseOutparam::set(response_out, Ok(response));
    }
}
