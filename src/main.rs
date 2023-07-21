mod deserialize;
mod request;
mod response;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use request::DeconstructedHTTPRequest;
use response::{HTTPError, Response};

const BUF_SIZE: usize = 1024;
async fn handle_connection(mut stream: TcpStream) {
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
    let response = match request_line.path.as_str() {
        "/hello" => Response::as_bytes(&format!("Hello, {}!", &request_line.path[1..])),
        "/notfound" => Response::as_bytes(&HTTPError::not_found()),
        "/internalservererror" => Response::as_bytes(&HTTPError::internal_server_error()),
        _ => Response::as_bytes("Default Response."),
    };
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
