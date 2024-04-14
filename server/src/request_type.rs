pub enum RequestType {
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    ALL,
}

impl RequestType {
    pub fn from_str(str: &str) -> RequestType {
        return match (str) {
            "GET" => RequestType::GET,
            "POST" => RequestType::POST,
            "HEAD" => RequestType::HEAD,
            "PUT" => RequestType::PUT,
            "DELETE" => RequestType::DELETE,
            "CONNECT" => RequestType::CONNECT,
            "TRACE" => RequestType::TRACE,
            "OPTIONS" => RequestType::OPTIONS,
            "PATCH" => RequestType::PATCH,
            _ => panic!("couldn't match given string to http verb"),
        };
    }
}

impl PartialEq for RequestType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RequestType::GET, RequestType::GET)
            | (RequestType::POST, RequestType::POST)
            | (RequestType::HEAD, RequestType::HEAD)
            | (RequestType::PUT, RequestType::PUT)
            | (RequestType::DELETE, RequestType::DELETE)
            | (RequestType::CONNECT, RequestType::CONNECT)
            | (RequestType::OPTIONS, RequestType::OPTIONS)
            | (RequestType::TRACE, RequestType::TRACE)
            | (RequestType::PATCH, RequestType::PATCH)
            | (RequestType::ALL, RequestType::ALL) => true,
            _ => false,
        }
    }
}
