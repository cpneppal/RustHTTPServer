use crate::{
    request::HTTPRequest,
    response::{HTTPError, Response, Result},
};

// import the Regex and Regex Error package
use regex::{Error, Regex};
use std::result;

#[derive(Debug)]
struct InternalRoute {
    method: String,
    path: Regex,
    http_version: String,
    callback: fn(HTTPRequest, Vec<u8>) -> Result<Box<dyn Response>>,
}

impl PartialEq<HTTPRequest> for InternalRoute {
    fn eq(&self, other: &HTTPRequest) -> bool {
        self.method == other.method
            && self.path.is_match_at(&other.path, 0)
            && self.http_version == other.http_version
    }
}

// This eq has symmetry as long as Internal route implements partial EQ on HTTP Request
impl PartialEq<InternalRoute> for HTTPRequest
where
    InternalRoute: PartialEq<Self>,
{
    fn eq(&self, other: &InternalRoute) -> bool {
        other.eq(self)
    }
}

pub struct Router {
    internal_route_vec: Vec<InternalRoute>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            internal_route_vec: Vec::new(),
        }
    }

    // Register a route in the router object. Consumes self and returns it back in either an Ok variant or an error when parsing the
    pub fn route(
        mut self,
        method: &str,
        path: &str,
        http_version: &str,
        callback: fn(HTTPRequest, Vec<u8>) -> Result<Box<dyn Response>>,
    ) -> result::Result<Self, Error> {
        self.internal_route_vec.push(InternalRoute {
            method: method.to_owned(),
            path: Regex::new(path)?,
            http_version: http_version.to_owned(),
            callback,
        });

        Ok(self)
    }

    pub fn handle_request(&self, request: HTTPRequest, body: Vec<u8>) -> Vec<u8> {
        self.internal_route_vec
            .iter()
            .find(|route| route == &&request)
            .ok_or(HTTPError::not_found())
            .and_then(|route| (route.callback)(request, body))
            .map(|result| result.to_response())
            .map_or_else(|error| error.to_response(), |success| success)
    }
}
