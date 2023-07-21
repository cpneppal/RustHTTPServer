use std::result;

/// The HTTP Result type.
pub type Result<T> = result::Result<T, HTTPError>;

pub trait Response {
    /// as_bytes takes a reference self into a vector of bytes that can be fed through a TCP Stream.
    fn to_response(&self) -> Vec<u8>;
}

/// HTTP Error may be 400 or 500 depending on the type of error. Assosiated functions are provided for common types.
#[derive(Debug)]
pub struct HTTPError {
    status_code: i32,
    message: String,
    body: String,
}

impl HTTPError {
    pub fn not_found() -> Self {
        HTTPError {
            status_code: 404,
            message: "Not found".to_owned(),
            body: "The requested content could not be found.".to_owned(),
        }
    }

    pub fn internal_server_error() -> Self {
        HTTPError {
            status_code: 500,
            message: "Internal Server Error".to_owned(),
            body: "The server has encountered an unexpected error.".to_owned(),
        }
    }
}

impl Response for HTTPError {
    fn to_response(&self) -> Vec<u8> {
        format!(
            "HTTP/1.1 {} {}\r\n\
            X-Content-Type-Options: nosniff\r\n\
            Content-Type: text/plain\r\n\
            Content-Length: {}\r\n\r\n\
            {}",
            self.status_code,
            self.message,
            self.body.len(),
            self.body
        )
        .into_bytes()
    }
}

/// This is a wrapper for returning HTML code
pub struct Html(String);

/// This is a wrapper for returning Javascript code
pub struct JavaScript(String);

/// This is a wrapper for returning CSS code
pub struct Css(String);

/// This is a wrapper for returning JSON
pub struct Json(String);

// Plain Text
impl Response for String {
    fn to_response(&self) -> Vec<u8> {
        // deref into &str and call its implementation
        self.as_str().to_response()
    }
}

// Plain Text
impl Response for str {
    fn to_response(&self) -> Vec<u8> {
        format!(
            "HTTP/1.1 200 OK\r\n\
            X-Content-Type-Options: nosniff\r\n\
            Content-Type: text/plain\r\n\
            Content-Length: {}\r\n\r\n\
            {}",
            self.len(),
            self
        )
        .into_bytes()
    }
}

// Plain Text
impl Response for &str {
    fn to_response(&self) -> Vec<u8> {
        Response::to_response(*self)
    }
}

// HTML
impl Response for Html {
    fn to_response(&self) -> Vec<u8> {
        todo!()
    }
}

// Javascript
impl Response for JavaScript {
    fn to_response(&self) -> Vec<u8> {
        todo!()
    }
}

// CSS
impl Response for Css {
    fn to_response(&self) -> Vec<u8> {
        todo!()
    }
}

// Javascript
impl Response for Json {
    fn to_response(&self) -> Vec<u8> {
        todo!()
    }
}
