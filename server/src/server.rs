use colored::Colorize;

use crate::{handler::Handler, ResponseStructure};
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    handlers: Vec<Handler>,
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new() -> Self {
        // let addr = "127.0.0.1:7878";

        let server = Server {
            handlers: vec![],
            listener: None,
        };

        return server;
    }

    pub fn listen(&mut self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        self.listener = Some(listener.try_clone().unwrap());
        println!("{}", format!("Listening on http://{addr}").bold().green());
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_request(stream)
        }
    }

    pub fn add_handler(&mut self, handler: Handler) {
        println!("added handler for {}", handler.path());
        self.handlers.push(handler);
    }

    pub fn handle_request(&self, mut stream: TcpStream) {
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
            let mut fpath = path.clone();

            // check if any handler handles this path
            for handler in &self.handlers {
                if handler.path().eq(&fpath) {
                    handler.handle(stream);
                    println!(
                        "{} ({}):\t{}",
                        "HANDLED".blue(),
                        handler.path().green(),
                        http_request.get(0).unwrap().blue()
                    );
                    // only one handler can handle a path
                    return;
                }
            }
            // else, try to return resource. else, 404.

            // first, check if path has a full stop
            let mut has_dot = false;
            for c in path.chars() {
                if c.eq(&'.') {
                    has_dot = true;
                    break;
                }
            }

            if !has_dot {
                fpath = fpath + ".html";
                println!(
                    "{}",
                    format!(
                        "{}",
                        "path does not contain full stop, appending .html".bright_blue()
                    )
                );
            }

            let contents = fs::read(format!("pages/{fpath}"));
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
        let _ = stream.write_all(response.as_slice());

        self.log_response(target, res.status, res.status_message);
    }

    fn log_response(&self, request: &String, res_status: i32, res_message: &str) {
        if res_status == 200 {
            println!(
                "{} {}:   {}",
                format!("{}", res_status).bold().green(),
                res_message.green(),
                request.blue()
            );
        } else {
            println!(
                "{} {}:  {}",
                format!("{}", res_status).bold().red(),
                res_message.red(),
                format!("{} (FAILED)", request).red(),
            );
        }
    }
}
