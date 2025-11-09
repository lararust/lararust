use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
}

impl Method {
    pub fn from_str(method: &str) -> Option<Self> {
        match method {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            "PUT" => Some(Method::PUT),
            "PATCH" => Some(Method::PATCH),
            "DELETE" => Some(Method::DELETE),
            "OPTIONS" => Some(Method::OPTIONS),
            "HEAD" => Some(Method::HEAD),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub body: Vec<u8>,
}

impl Request {
    pub fn from_raw(raw: &str) -> Option<Self> {
        let mut lines = raw.lines();

        let request_line = lines.next()?;

        let mut parts = request_line.split_whitespace();
        let method_str = parts.next()?;
        let path = parts.next()?.to_string();

        let method = Method::from_str(method_str)?;

        let mut headers = HashMap::new();

        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            if let Some((key, value)) = line.split_once(":") {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        let body = lines
            .collect::<Vec<&str>>()
            .join("\n")
            .into_bytes();

        Some(Self { method, path, body })
    }
}
