use colored::Colorize;
use handler::Handler;
use server::Server;
use std::{io::Write, net::TcpStream};

mod handler;
mod server;

struct ResponseStructure<'a> {
    status: i32,
    status_message: &'a str,
    data: Vec<u8>,
}

fn main() {
    let mut server: Server = Server::new();

    // server.add_handler(Handler::new(
    //     "/about".to_string(),
    //     Some(Box::new(|stream: TcpStream| {
    //         println!("{}", "Handled request to '/about'!".blue());
    //         stream.write_all(buf)
    //     })),
    // ));

    server.add_handler(Handler::for_resource(
        "/about".to_string(),
        "pages/mysillypage.html",
    ));

    server.listen("127.0.0.1:7878");
}
