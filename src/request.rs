use std::str::{from_utf8, FromStr};

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct HTTPRequest {
    method: String,
    path: String,
    http_version: String,
    content_length: Option<usize>,
    content_type: Option<String>,
}

impl<'a> TryFrom<&'a [u8]> for HTTPRequest {
    type Error = String;
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let mut stop_point: usize = 0;
        // look for first occurence of \r\n\r\n
        for i in 0..=value.len() {
            let four_byte_slice = &value[i..i + 4];
            if four_byte_slice == b"\r\n\r\n" {
                stop_point = i;
                break;
            } else if i >= value.len() - 4 {
                // bounds check. If we are at the end of the valuefer, break.
                return Err(
                    r"Could not find the \r\n\r\n that seperates the headers from the body"
                        .to_owned(),
                );
            }
        }
        // Convert from UTF 8, map the error, then use and_then to try to convert from a string and return a result with findings.
        // Cannot use map as that will wrap the from str result within a result resulting in nested results.
        from_utf8(&value[..=stop_point])
            .map_err(|_| "Could not convert byte sequence to UTF-8".to_owned())
            .and_then(Self::from_str)
    }
}

impl FromStr for HTTPRequest {
    type Err = String;
    // Input: s as a request line, up to the first \r\n\r\n
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_line, _rest) = s
            .split_once("\r\n")
            .ok_or("Could not find first line of HTTP Request")?;

        let re = Regex::new(r"([A-Z]+)\s+(/[^\s]*)\s+HTTP/(\d\.\d)")
            .map_err(|_| "Could not get regex to parse first line".to_owned())?;

        let (_, [method, path, http_version]) =
            re.captures(first_line).map(|s| s.extract()).ok_or(
                "Regex for first line compiled, but couldn't find the methods on the first line",
            )?;

        Ok(HTTPRequest {
            method: method.to_owned(),
            path: path.to_owned(),
            http_version: http_version.to_owned(),
            content_length: None,
            content_type: None,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn valid_get_request() {
        let test_bytes: &[u8] = &[
            71, 69, 84, 32, 47, 32, 72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 97, 99, 99, 101, 112,
            116, 45, 101, 110, 99, 111, 100, 105, 110, 103, 58, 32, 103, 122, 105, 112, 44, 32,
            100, 101, 102, 108, 97, 116, 101, 44, 32, 98, 114, 13, 10, 65, 99, 99, 101, 112, 116,
            58, 32, 42, 47, 42, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116, 58, 32, 84,
            104, 117, 110, 100, 101, 114, 32, 67, 108, 105, 101, 110, 116, 32, 40, 104, 116, 116,
            112, 115, 58, 47, 47, 119, 119, 119, 46, 116, 104, 117, 110, 100, 101, 114, 99, 108,
            105, 101, 110, 116, 46, 99, 111, 109, 41, 13, 10, 72, 111, 115, 116, 58, 32, 108, 111,
            99, 97, 108, 104, 111, 115, 116, 58, 56, 48, 56, 48, 13, 10, 67, 111, 110, 110, 101,
            99, 116, 105, 111, 110, 58, 32, 99, 108, 111, 115, 101, 13, 10, 13, 10, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let expected_answer: HTTPRequest = HTTPRequest {
            method: "GET".to_owned(),
            path: "/".to_owned(),
            http_version: "1.1".to_owned(),
            content_length: None,
            content_type: None,
        };
        let actual_answer: HTTPRequest = test_bytes
            .try_into()
            .expect("Could not convert byte slice into HTTP Request");

        assert_eq!(expected_answer, actual_answer);
    }
    #[test]
    fn test_valid_get_request_2() {
        let test_bytes: &[u8] = &[
            71, 69, 84, 32, 47, 104, 101, 108, 108, 111, 32, 72, 84, 84, 80, 47, 49, 46, 49, 13,
            10, 72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 56, 48,
            56, 48, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116, 58, 32, 77, 111, 122,
            105, 108, 108, 97, 47, 53, 46, 48, 32, 40, 87, 105, 110, 100, 111, 119, 115, 32, 78,
            84, 32, 49, 48, 46, 48, 59, 32, 87, 105, 110, 54, 52, 59, 32, 120, 54, 52, 59, 32, 114,
            118, 58, 49, 48, 57, 46, 48, 41, 32, 71, 101, 99, 107, 111, 47, 50, 48, 49, 48, 48, 49,
            48, 49, 32, 70, 105, 114, 101, 102, 111, 120, 47, 49, 49, 53, 46, 48, 13, 10, 65, 99,
            99, 101, 112, 116, 58, 32, 116, 101, 120, 116, 47, 104, 116, 109, 108, 44, 97, 112,
            112, 108, 105, 99, 97, 116, 105, 111, 110, 47, 120, 104, 116, 109, 108, 43, 120, 109,
            108, 44, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 47, 120, 109, 108, 59,
            113, 61, 48, 46, 57, 44, 105, 109, 97, 103, 101, 47, 97, 118, 105, 102, 44, 105, 109,
            97, 103, 101, 47, 119, 101, 98, 112, 44, 42, 47, 42, 59, 113, 61, 48, 46, 56, 13, 10,
            65, 99, 99, 101, 112, 116, 45, 76, 97, 110, 103, 117, 97, 103, 101, 58, 32, 101, 110,
            45, 85, 83, 44, 101, 110, 59, 113, 61, 48, 46, 53, 13, 10, 65, 99, 99, 101, 112, 116,
            45, 69, 110, 99, 111, 100, 105, 110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101,
            102, 108, 97, 116, 101, 44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105,
            111, 110, 58, 32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 85, 112, 103,
            114, 97, 100, 101, 45, 73, 110, 115, 101, 99, 117, 114, 101, 45, 82, 101, 113, 117,
            101, 115, 116, 115, 58, 32, 49, 13, 10, 83, 101, 99, 45, 70, 101, 116, 99, 104, 45, 68,
            101, 115, 116, 58, 32, 100, 111, 99, 117, 109, 101, 110, 116, 13, 10, 83, 101, 99, 45,
            70, 101, 116, 99, 104, 45, 77, 111, 100, 101, 58, 32, 110, 97, 118, 105, 103, 97, 116,
            101, 13, 10, 83, 101, 99, 45, 70, 101, 116, 99, 104, 45, 83, 105, 116, 101, 58, 32,
            110, 111, 110, 101, 13, 10, 83, 101, 99, 45, 70, 101, 116, 99, 104, 45, 85, 115, 101,
            114, 58, 32, 63, 49, 13, 10, 13, 10, 0, 0,
        ];
        let expected_answer: HTTPRequest = HTTPRequest {
            method: "GET".to_owned(),
            path: "/hello".to_owned(),
            http_version: "1.1".to_owned(),
            content_length: None,
            content_type: None,
        };
        let actual_answer: HTTPRequest = test_bytes
            .try_into()
            .expect("Could not convert byte slice into HTTP Request");

        assert_eq!(expected_answer, actual_answer);
    }
}
