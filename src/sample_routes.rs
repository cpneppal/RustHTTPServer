use http::{HTTPRequest, HTTPResponses, Result, Router};

pub fn http_routes() -> Router {
    Router::new()
        .route("GET", "/$", "1.1", hello_world)
        .and_then(|route| route.route("POST", "/image$", "1.1", get_image))
        .and_then(|route| route.route("POST", "/user_json$", "1.1", print_json))
        .unwrap()
}

fn hello_world(_headers: HTTPRequest, _body: Vec<u8>) -> Result<HTTPResponses> {
    Ok("Hello, world!".into())
}

fn get_image(_headers: HTTPRequest, body: Vec<u8>) -> Result<HTTPResponses> {
    println!("Body Length: {}", body.len());
    Ok("Image Received!".into())
}

fn print_json(_headers: HTTPRequest, body: Vec<u8>) -> Result<HTTPResponses> {
    println!(
        "Json Receieved: {}",
        String::from_utf8(body).map_err(|err| {
            eprintln!("Could not format the body as JSON => {err}");
            HTTPResponses::internal_server_error()
        })?
    );
    Ok("Received Json!".into())
}
