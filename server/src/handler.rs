use std::{fs, io::Write, net::TcpStream};

pub struct Handler {
    path: String,
    cb: Option<Box<dyn Fn(TcpStream) -> ()>>,
}

impl Handler {
    pub fn new(path: String, cb: Option<Box<dyn Fn(TcpStream) -> ()>>) -> Self {
        println!("created handler for {}", path);
        Handler { path, cb }
    }

    pub fn for_resource(path: String, resource_path: &str) -> Self {
        let rpath = resource_path.to_string();
        let handler = move |mut stream: TcpStream| {
            let content = fs::read(rpath.clone());
            let status = if let Ok(_) = content { 200 } else { 404 };
            let mut response = format!(
                "HTTP/1.1 {} {}\r\n\r\n",
                status,
                if status == 200 {
                    "Resource Found"
                } else {
                    "Not Found"
                }
            )
            .into_bytes();
            response.extend(if let Ok(c) = content {
                c
            } else {
                fs::read("pages/404.html").unwrap()
            });
            let _ = stream.write_all(response.as_slice());
        };
        Handler {
            path,
            cb: Some(Box::new(handler)),
        }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn handle(&self, request: TcpStream) -> bool {
        if let Some(ref cb) = self.cb {
            (cb)(request);
            return true;
        }

        false
    }
}
