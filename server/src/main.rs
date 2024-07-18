use handler::Handler;
use request_type::RequestType;
use server::Server;
use srvr::ThreadPool;
use std::{net::TcpStream, sync::Arc};

mod handler;
mod request_type;
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

    server.add_handler(Handler::for_resource("/".to_string(), "pages/index.html"));

    server.add_handler(Handler::for_resource(
        "/about".to_string(),
        "pages/mysillypage.html",
    ));

    server.add_handler(Handler::new(
        "/submit".to_string(),
        Some(Arc::new(|mut _stream: TcpStream| {
            println!("HTTP/1.1 200 OK\r\nGot a post reqest to /submit!!1!\r\n");
        })),
        &[RequestType::POST],
    ));

    server.listen("10.0.0.15:7878");
}
