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
    pub fn from_str(str: &str) -> Result<RequestType, &'static str> {
        return match str {
            "GET" => Ok(RequestType::GET),
            "POST" => Ok(RequestType::POST),
            "HEAD" => Ok(RequestType::HEAD),
            "PUT" => Ok(RequestType::PUT),
            "DELETE" => Ok(RequestType::DELETE),
            "CONNECT" => Ok(RequestType::CONNECT),
            "TRACE" => Ok(RequestType::TRACE),
            "OPTIONS" => Ok(RequestType::OPTIONS),
            "PATCH" => Ok(RequestType::PATCH),
            _ => Err("Invalid request type"),
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
