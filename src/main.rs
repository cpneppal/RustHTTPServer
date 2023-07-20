mod request;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const BUF_SIZE: usize = 1024;

async fn handle_connection(mut stream: TcpStream) {
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

    let request_size = stream
        .read(&mut buf)
        .await
        .expect("Could not read from stream!");

    let request_line = request::HTTPRequest::try_from(buf.as_slice())
        .expect("Could not convert buffer to HTTP Request");

    println!("Request Size: {}\n{}\n", request_size, request_line);
    let response: &str = "HTTP/1.1 200 OK\r\n\r\n";
    stream
        .write_all(response.as_bytes())
        .await
        .expect("Error writing response");

    println!("Success!");
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
