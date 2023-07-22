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
                // return the image
                Ok(Image {
                    ext: "jpg".to_owned(),
                    content: body,
                })
            })
        })
        .unwrap()
}
