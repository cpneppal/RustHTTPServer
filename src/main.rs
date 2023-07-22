mod sample_routes;
use http::{DeconstructedHTTPRequest, Router};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const BUF_SIZE: usize = 1024;
async fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

    let request_size = stream
        .read(buf.as_mut_slice())
        .await
        .expect("Could not read from stream!");

    let DeconstructedHTTPRequest(request_line, body_start) = buf
        .as_slice()
        .try_into()
        .expect("Could not convert buffer to HTTP Request");

    println!("Request Line => {request_line:?}");

    // allocate a vector of bytes that has a capcity of the content length if it exists or 0
    let mut body: Vec<u8> = Vec::with_capacity(request_line.content_length.unwrap_or_default());

    // finish the stream if body length < content_length
    if let Some(content_length) = request_line.content_length {
        body.extend_from_slice(&buf[body_start..request_size]);
        while body.len() < content_length {
            let request_size = stream
                .read(buf.as_mut_slice())
                .await
                .expect("Could not read from stream!");

            body.extend_from_slice(&buf[..request_size]);
        }
    }
    println!("Body Length => {}", body.len());
    let response = router.handle_request(request_line, body);
    stream
        .write_all(response.as_slice())
        .await
        .expect("Error writing response");
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Error binding to tcp socket.");

    let router: Arc<Router> = Arc::new(Router::new().with(sample_routes::http_routes()));

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Error unwraping the listener");
        let routeref = Arc::clone(&router);
        tokio::spawn(async move {
            handle_connection(socket, routeref).await;
        });
    }
}
