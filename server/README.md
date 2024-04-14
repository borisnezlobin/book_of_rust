# srvr

## about

My little multithreaded rust server, used for learning rust. Took the base from the book of rust's server project, then added handlers, HTTP verb handling, a real implementation of the server, and made a little static site (that, in theory, doesn't even have to be static.)

## usage

To use, simply create a `Server`:

```rust
let mut server: Server = Server::new();
```

Then, for every path/route that needs handling, create handlers. There are a few ways to do so:

## 1. for resource

Will handle GET requests to a certain path by reading a file and sending it back:

```rust
server.add_handler(Handler::for_resource("/".to_string(), "pages/index.html"));

server.add_handler(Handler::for_resource(
    "/about".to_string(),
    "pages/mysillypage.html",
));
```

This will create handlers that will respond with the contents of `pages/index.html` and `pages/mysillypage.html` to `GET` requests to `/` and `/about`, respectively.

## 2. for everything else

For more detailed request/response handling (such as handling requests other than `GET`), create a handler with a closure:

```rust
server.add_handler(Handler::new(
    "/submit".to_string(),
    Some(Box::new(|mut stream: TcpStream| {
        println!("Got a post reqest!!1!");
    })),
    &[RequestType::POST],
));
```

Note that this is still being worked on, so it is necessary to format your HTTP response yourself.

## 3. default

If no other handlers are matched for a specific request, then the server will try to respond with the contents of `/pages/{request path}`... but for some reason (I really don't know why), it is immune to path traversal attacks. :)
