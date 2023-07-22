mod request;
mod response;
mod route;

pub use request::{DeconstructedHTTPRequest, HTTPRequest};
pub use response::{HTTPResponses, Response, Result};
pub use route::Router;
use std::fs;
pub use HTTPResponses::*;
