use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum HTTPTag {
    HOST,
    CONNECTION,
    CacheControl,
    UPGRADE,
    UpgradeInsecureRequests,
    UserAgent,
    AcceptEncoding,
    AcceptLanguage,
    SecWebSocketKey,

    UNDEFINED,
}

impl HTTPTag {
    pub fn from_string(s: &str) -> Result<HTTPTag, &str> {
        match s {
            "Host:" => Ok(HTTPTag::HOST),
            "Upgrade:" => Ok(HTTPTag::UPGRADE),
            "User-Agent:" => Ok(HTTPTag::UserAgent),
            "Connection:" => Ok(HTTPTag::CONNECTION),
            "Cache-Control:" => Ok(HTTPTag::CacheControl),
            "Accept-Encoding:" => Ok(HTTPTag::AcceptEncoding),
            "Accept-Language:" => Ok(HTTPTag::AcceptLanguage),
            "Sec-WebSocket-Key:" => Ok(HTTPTag::SecWebSocketKey),
            "Upgrade-Insecure-Requests:" => Ok(HTTPTag::UpgradeInsecureRequests),
            _ => Ok(HTTPTag::UNDEFINED),
        }
    }
}

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
}

impl Method {
    pub fn from_string(s: &str) -> Result<Method, &str> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            _ => Err("Failed to parse method from string"),
        }
    }
}

pub struct HTTPRequest {
    pub method: Method,
    pub headers: HashMap<HTTPTag, String>,
}

impl HTTPRequest {
    pub fn new(request: &str) -> Self {
        let lines: Vec<&str> = request.split('\n').collect();
        let method = Method::from_string(
            lines
                .first()
                .expect("Could not get first line")
                .split(" ")
                .nth(0)
                .expect("Could not get element"),
        )
        .expect("Could not parse method");

        let mut headers: HashMap<HTTPTag, String> = HashMap::new();
        for l in &lines {
            let mut s = l.split(" ");
            let key = s.nth(0);

            let mut temp = String::new();
            let mut words = s.clone();
            let value = if s.count() > 1 {
                for w in words {
                    if temp.len() > 0 {
                        temp = format!("{temp} {w}");
                    } else {
                        temp = w.to_string();
                    }
                }
                Some(temp.as_str())
            } else {
                words.nth(0)
            };

            if key.is_some() && value.is_some() {
                let key = HTTPTag::from_string(key.expect("Could not split value"))
                    .expect("Could not parse into tag");
                let value = String::from(value.expect("Could not split value correctly").trim());
                headers.insert(key, value);
            }
        }
        //Just for debug
        for l in &lines {
            if !l.starts_with("\0\0") {
                println!("{}", l);
            }
        }
        Self { method, headers }
    }

    pub fn get_header_value_key(&mut self, key: HTTPTag) -> Option<String> {
        self.headers.remove(&key)
    }
}
