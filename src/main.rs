mod deserialize;
mod request;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use request::DeconstructedHTTPRequest;

const BUF_SIZE: usize = 1024;
async fn handle_connection(mut stream: TcpStream) {
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

    let request_size = stream
        .read(&mut buf)
        .await
        .expect("Could not read from stream!");

    let DeconstructedHTTPRequest(request_line, stop_point) = buf
        .as_slice()
        .try_into()
        .expect("Could not convert buffer to HTTP Request");

    let body = &buf[stop_point + 4..request_size];

    println!("Request Line => {request_line:?}");

    let mut total_body: Vec<u8> = Vec::from(body);

    // finish the stream if body length < content_length
    if let Some(content_length) = request_line.content_length {
        while total_body.len() < content_length {
            let request_size = stream
                .read(&mut buf)
                .await
                .expect("Could not read from stream!");

            total_body.extend_from_slice(&buf[..request_size]);
        }
    }
    println!("Body Length => {}", total_body.len());
    let response: &str = "HTTP/1.1 200 OK\r\n\r\n";
    stream
        .write_all(response.as_bytes())
        .await
        .expect("Error writing response");
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Error binding to tcp socket.");

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Error unwraping the listener");

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}
