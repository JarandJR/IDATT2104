use crate::http_parser::{HTTPRequest, HTTPTag};

pub struct SocketRequest {
    http_request: HTTPRequest,
    sec_key: String,
    GUID: String,
}

impl SocketRequest {
    pub fn new(mut http_request: HTTPRequest) -> Self {
        let header_value = http_request.get_header_value_key(HTTPTag::SecWebSocketKey);
        Self {
            http_request,
            sec_key: header_value,
            GUID: String::from("insert key here"),
        }
    }
}
