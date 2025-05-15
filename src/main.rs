mod response;
mod request;
mod handler;

use crate::request::Request;

use std::{
    io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}
};

fn main() {

    let args = std::env::args().collect::<Vec<_>>();

    let mut port = 7474;

    if args.len() > 0 && args[1].parse::<u16>().is_ok() {
        port = args[1].parse::<u16>().unwrap();
    }

    let addr = format!("127.0.0.1:{}", port); 

    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let response = Request::new(request_line).parse();
    
    stream.write_all(response.to_string().as_bytes()).unwrap();
}

