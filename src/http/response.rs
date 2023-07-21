use std::result;
use std::str::FromStr;
/// The HTTP Result type.
pub type Result<T> = result::Result<T, HTTPResponses>;

pub trait Response {
    /// Into Response consumes self and returns a vector of bytes as a TCP stream.
    /// Intended to be flexible with future versions of Responses that may not be of HTTP
    fn to_response(self) -> Vec<u8>;
}

/// Common response types for plain text, hmtl, javascript and so on.
#[derive(Debug)]
pub enum HTTPResponses {
    PlainText(String),
    Html(String),
    JavaScript(String),
    Css(String),
    Json(String),
    Redirect(String),
    HTTPError {
        status_code: i32,
        message: String,
        body: String,
    },
}

/// When converting to response, if statements handle special cases. For instance, redirect's HTTP status code is different from the rest, so it needs to be handled separatley. This helps to avoid writing duplicate code.
impl Response for HTTPResponses {
    fn to_response(self) -> Vec<u8> {
        // handle the redirect case separate
        if let Self::Redirect(s) = self {
            format!(
                "HTTP/1.1 301 Moved Permanently\r\n\
                X-Content-Type-Options: nosniff\r\n\
                Location: {s}\r\n\r\n"
            )
            .into_bytes()
        } else {
            let (code, message, ctype, length, content) = match self {
                Self::PlainText(s) => (200, "OK".to_owned(), "text/plain", s.len(), s),
                Self::Html(s) => (200, "OK".to_owned(), "text/html; charset=utf-8", s.len(), s),
                Self::JavaScript(s) => (200, "OK".to_owned(), "text/javascript", s.len(), s),
                Self::Css(s) => (200, "OK".to_owned(), "text/css", s.len(), s),
                Self::Json(s) => (200, "OK".to_owned(), "application/json", s.len(), s),
                Self::HTTPError {
                    status_code,
                    message,
                    body,
                } => (status_code, message, "text/plain", body.len(), body),
                _ => panic!("Unreachable!"),
            };

            format!(
                "HTTP/1.1 {code} {message}\r\n\
                X-Content-Type-Options: nosniff\r\n\
                Content-Type: {ctype}\r\n\
                Content-Length: {length}\r\n\r\n\
                {content}"
            )
            .into_bytes()
        }
    }
}
impl HTTPResponses {
    pub fn not_found() -> Self {
        Self::HTTPError {
            status_code: 404,
            message: "Not found".to_owned(),
            body: "The requested content could not be found.".to_owned(),
        }
    }

    pub fn internal_server_error() -> Self {
        Self::HTTPError {
            status_code: 500,
            message: "Internal Server Error".to_owned(),
            body: "The server has encountered an unexpected error.".to_owned(),
        }
    }
}

// Plain Text as (as String)
impl From<String> for HTTPResponses {
    fn from(value: String) -> Self {
        Self::PlainText(value)
    }
}
// Plain Text (as &str)
impl From<&str> for HTTPResponses {
    fn from(value: &str) -> Self {
        // This can never fail, so call unwrap without worry
        Self::from_str(value).unwrap()
    }
}

// Plain Text (as string)
impl FromStr for HTTPResponses {
    type Err = String;
    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        Ok(Self::PlainText(s.to_owned()))
    }
}
