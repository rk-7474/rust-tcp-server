
use crate::response::Response;

use std::{
    fs, path::Path
};

pub fn handle_get_request(request_line: &str) -> Response {
    let mut path = format!("./res{}", request_line.trim().split(" ").nth(1).unwrap());
    
    if path.ends_with("/") {
        path.push_str("index.html");
    }

    if Path::new(&path).is_file() {
        Response::new(fs::read_to_string(path).unwrap(), 200, "OK".to_string())
    } else {
        Response::new(format!("Error: Path {} not found", path), 404, "Not Found".to_string()) 
    }
}