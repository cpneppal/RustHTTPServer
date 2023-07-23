use http::{
    http_err, http_ok, HTTPRequest,
    HTTPResponses::{self, *},
    HTTPResult, Router,
};
pub fn http_routes() -> Router {
    Router::new()
        .route("GET|POST", "/$", "1.1", hello_world)
        .and_then(|route| route.route("POST", "/image$", "1.1", get_image))
        .and_then(|route| route.route("POST", "/user_json$", "1.1", print_json))
        .and_then(|route| route.route("GET|POST", "/custom$", "1.1", custom_route))
        .unwrap()
}

fn hello_world(_headers: HTTPRequest, _body: Vec<u8>) -> HTTPResult {
    http_ok("Hello, world!".into())
}

fn custom_route(headers: HTTPRequest, _: Vec<u8>) -> HTTPResult {
    println!("Headers => {headers:?}");
    http_ok(Custom {
        code: 201,
        message: "Created".to_owned(),
        ctype: "text/plain".to_owned(),
        headers: None,
        body: Vec::from("Something has occured!".as_bytes()),
    })
}

fn get_image(_headers: HTTPRequest, body: Vec<u8>) -> HTTPResult {
    println!("Body Length: {}", body.len());
    http_ok(Redirect("/".to_owned()))
}

fn print_json(_headers: HTTPRequest, body: Vec<u8>) -> HTTPResult {
    println!(
        "Json Receieved: {}",
        String::from_utf8(body).map_err(|err| {
            eprintln!("Could not format the body as JSON => {err}");
            HTTPResponses::internal_server_error()
        })?
    );
    http_err(HTTPError {
        status_code: 500,
        message: "Some Server Error".to_owned(),
        body: "Congrats, you've broken our site!".to_owned(),
    })
}
