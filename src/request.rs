use crate::response::Response;
use std::{fs, path::Path};
use crate::handler;

enum RequestType {
    GET,
    INVALID
}

pub struct Request {
    content: String,
    request_type: RequestType,    
}

impl Request {
    pub fn new(request_line: String) -> Request {
        let request_type = match request_line.split(" ").nth(0).unwrap() {
            "GET" => RequestType::GET,
            _ => RequestType::INVALID
        };

        Request {
            content: request_line,
            request_type
        }
    }

    pub fn parse(&self) -> Response {
        match self.request_type {
            RequestType::GET => handler::handle_get_request(&self.content),
            RequestType::INVALID => Response::new("Error: Invalid request".to_string(), 400, "Bad Request".to_string())
        }
    }
}