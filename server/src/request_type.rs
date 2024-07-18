#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod request_type_tests {
    #[test]
    fn test_request_type_from_str() {
        use super::RequestType;
        assert_eq!(RequestType::from_str("GET").unwrap(), RequestType::GET);
        assert_eq!(RequestType::from_str("POST").unwrap(), RequestType::POST);
        assert_eq!(RequestType::from_str("HEAD").unwrap(), RequestType::HEAD);
        assert_eq!(RequestType::from_str("PUT").unwrap(), RequestType::PUT);
        assert_eq!(RequestType::from_str("DELETE").unwrap(), RequestType::DELETE);
        assert_eq!(RequestType::from_str("CONNECT").unwrap(), RequestType::CONNECT);
        assert_eq!(RequestType::from_str("TRACE").unwrap(), RequestType::TRACE);
        assert_eq!(RequestType::from_str("OPTIONS").unwrap(), RequestType::OPTIONS);
        assert_eq!(RequestType::from_str("PATCH").unwrap(), RequestType::PATCH);
        assert_eq!(RequestType::from_str("INVALID").is_err(), true);
    }

    #[test]
    fn test_eq() {
        use super::RequestType;
        assert_eq!(RequestType::GET, RequestType::GET);
        assert_eq!(RequestType::POST, RequestType::POST);
        assert_eq!(RequestType::HEAD, RequestType::HEAD);
        assert_eq!(RequestType::PUT, RequestType::PUT);
        assert_eq!(RequestType::DELETE, RequestType::DELETE);
        assert_eq!(RequestType::CONNECT, RequestType::CONNECT);
        assert_eq!(RequestType::TRACE, RequestType::TRACE);
        assert_eq!(RequestType::OPTIONS, RequestType::OPTIONS);
        assert_eq!(RequestType::PATCH, RequestType::PATCH);
        assert_eq!(RequestType::ALL, RequestType::ALL);
    }

    #[test]
    fn test_neq() {
        use super::RequestType;
        assert_ne!(RequestType::GET, RequestType::POST);
        assert_ne!(RequestType::GET, RequestType::HEAD);
        assert_ne!(RequestType::GET, RequestType::PUT);
        assert_ne!(RequestType::GET, RequestType::DELETE);
        assert_ne!(RequestType::GET, RequestType::CONNECT);
        assert_ne!(RequestType::GET, RequestType::TRACE);
        assert_ne!(RequestType::GET, RequestType::OPTIONS);
        assert_ne!(RequestType::GET, RequestType::PATCH);
        assert_ne!(RequestType::GET, RequestType::ALL);
    }
}
