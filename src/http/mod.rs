mod request;
mod response;
mod route;

pub use request::DeconstructedHTTPRequest;
pub use response::{HTTPResponses, Response, Result};
pub use route::Router;
use std::fs;
pub use HTTPResponses::*;
pub fn http_routes() -> Router {
    Router::new()
        .route("GET", r"/(\d+)$", "1.1", |_, _| Ok("Hello, World!".into()))
        .and_then(|r| {
            r.route("POST", r"/$", "1.1", |_, body| {
                //println!("Body => {body:?}");
                fs::write("result.jpeg", body)
                    .map_err(|_| HTTPResponses::internal_server_error())?;
                Ok("Succeeded in writing image!".into())
            })
        })
        .unwrap()
}
