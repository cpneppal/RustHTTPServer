mod request;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const BUF_SIZE: usize = 1024;
use std::str::from_utf8;

async fn handle_connection(mut stream: TcpStream) {
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

    let request_size = stream
        .read(&mut buf)
        .await
        .expect("Could not read from stream!");

    let mut stop_point: usize = 0;
    // look for first occurence of \r\n\r\n
    for i in 0..=BUF_SIZE {
        let four_byte_slice = &buf[i..i + 4];
        if four_byte_slice == b"\r\n\r\n" {
            stop_point = i;
            break;
        } else if i >= BUF_SIZE - 4 {
            // bounds check. If we are at the end of the buffer, break.
            break;
        }
    }

    let request_line = from_utf8(&buf[..=stop_point]).expect("Could not convert to UTF-8!");

    let body = &buf[..];

    println!("Body: {:?}", body);
    println!(
        "Request Size: {}\nRequest Line: {}\n",
        request_size, request_line
    );
    let response: &str = "HTTP/1.1 200 OK\r\n\r\n";
    stream
        .write_all(response.as_bytes())
        .await
        .expect("Error writing response");

    println!("Success!");
    // --snip--
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
