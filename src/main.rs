mod deserialize;
mod request;
use std::str::from_utf8;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use deserialize::Color;
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

    // Get Json and filter it, removing spaces and new lines
    // TODO: See if you can use and_then and map_err to merge the two result types into 1
    let body = from_utf8(body).expect("Could not convert body byte sequence to UTF-8.");
    let body: Vec<Color> = serde_json::from_str(body).expect("Couldn't parse JSON");
    println!("Request Size: {}\n{}\n", request_size, request_line);
    println!("Body Length: {}\nBody: {:?}", body.len(), body);
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
