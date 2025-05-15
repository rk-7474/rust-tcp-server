mod response;
mod request;
mod handler;

use crate::request::Request;

use std::{
    env, fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, path::Path
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Request type: {}", request_line);

    let response = Request::new(request_line).parse();
    
    stream.write_all(response.to_string().as_bytes()).unwrap();
}

