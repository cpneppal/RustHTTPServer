mod deserialize;
mod request;
mod response;
mod route;

use route::Router;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use request::DeconstructedHTTPRequest;

const BUF_SIZE: usize = 1024;
async fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

    let request_size = stream
        .read(buf.as_mut_slice())
        .await
        .expect("Could not read from stream!");

    let DeconstructedHTTPRequest(request_line, stop_point) = buf
        .as_slice()
        .try_into()
        .expect("Could not convert buffer to HTTP Request");

    println!("Request Line => {request_line:?}");

    let mut body: Vec<u8> = Vec::from(&buf[stop_point + 4..request_size]);

    // finish the stream if body length < content_length
    if let Some(content_length) = request_line.content_length {
        while body.len() < content_length {
            let request_size = stream
                .read(buf.as_mut_slice())
                .await
                .expect("Could not read from stream!");

            body.extend_from_slice(&buf[..request_size]);
        }
    }
    println!("Body Length => {}", body.len());
    println!("Body => {body:?}");
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

    let router: Arc<Router> = Arc::new(
        Router::new()
            .route("GET", r"(\d+)$", "1.1", |_, _| {
                Ok(Box::new("Hello, World!"))
            })
            .unwrap(),
    );

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
