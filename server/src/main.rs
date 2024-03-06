use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod server;

struct ResponseStructure<'a> {
    status: i32,
    status_message: &'a str,
    data: Vec<u8>,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let target = http_request.get(0).expect("Invalid HTTP request.");
    let split: Vec<String> = target.split(" ").map(|s| s.to_string()).collect();
    let mut res: ResponseStructure = ResponseStructure {
        status: 404,
        status_message: "Not Found",
        data: fs::read("pages/404.html").unwrap(),
    };
    if let Some(path) = split.get(1) {
        let contents = fs::read(format!("pages/{path}"));
        if let Ok(content) = contents {
            res = ResponseStructure {
                status: 200,
                status_message: "Resource Found",
                data: content,
            };
        }
    }

    let mut response =
        format!("HTTP/1.1 {} {}\r\n\r\n", res.status, res.status_message).into_bytes();

    response.extend(res.data);
    stream.write_all(response.as_slice());

    println!("{}: responded with {}", target, res.status);
}
