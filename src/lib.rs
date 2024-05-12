use log::{warn, error};
use std::fmt;

#[derive(Clone)]
pub struct Header {
    name: String,
    value: String,
}
impl Header {
    pub fn new(name: String, value: String) -> Header {
	Header {
	    name: name,
	    value: value,
	}
    }

    pub fn name(&self) -> &String {
	&self.name
    }

    pub fn value(&self) -> &String {
	&self.value
    }
}

#[derive(Clone)]
pub struct Query {
    name: String,
    value: String,
}
impl Query {
    pub fn new(name: String, value: String) -> Query {
	Query {
	    name: name,
	    value: value,
	}
    }
    
    pub fn name(&self) -> &String {
	&self.name
    }
    
    pub fn value(&self) -> &String {
	&self.value
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    PATCH,
}
impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Method::GET => write!(f, "GET"),
	    Method::POST => write!(f, "POST"),
	    Method::PUT => write!(f, "PUT"),
	    Method::DELETE => write!(f, "DELETE"),
	    Method::HEAD => write!(f, "HEAD"),
	    Method::OPTIONS => write!(f, "OPTIONS"),
	    Method::CONNECT => write!(f, "CONNECT"),
	    Method::TRACE => write!(f, "TRACE"),
	    Method::PATCH => write!(f, "PATCH"),
	}
    }
}

#[derive(Clone)]
pub struct Request {
    headers: Vec<Header>,
    query: Vec<Query>,
    body: String,
    method: Method,
    full_path: String,
    path: String,
    initialized: bool,
}
impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	if !self.initialized {
	    return write!(f, "Request read not initialized");
	}
	
	let mut headers: String = String::new();
	if self.headers.len() > 0 {
	    headers.push_str(&format!("\x1B[1mHeaders:\n\x1B[0m"));
	    for header in &self.headers {
		headers.push_str(&format!("  \"{}\": \"{}\"\r\n", header.name(), header.value()));
	    }
	}

	let mut query_str: String = String::new();
 	if self.query.len() > 0 {
	    query_str.push_str(&format!("\x1B[1mQueries:\n\x1B[0m"));
	    for query in &self.query {
		query_str.push_str(&format!("  \"{}\" = \"{}\"\n", query.name(), query.value()));
	    }
	}

	let mut body_str: String = String::new();
	if self.body.len() > 0 {
	    body_str.push_str(&format!("\x1B[1mBody:\n\x1B[0m  \""));
	    body_str.push_str(&self.body.clone());
	    body_str.push_str(&format!("\""));
	}
	
        write!(f, "\x1B[1mRequest:\x1B[0m\n  {} {}\n{}{}{}", self.method, self.path, headers, query_str, body_str)
    }
}
impl Request {
    pub fn new() -> Request {
	Request {
	    headers: Vec::new(),
	    query: Vec::new(),
	    body: String::new(),
	    method: Method::GET,
	    path: String::new(),
	    full_path: String::new(),
	    initialized: false,
	}
    }

    pub fn headers(&self) -> &Vec<Header> {
	if !self.initialized {
	    warn!("Request headers read not initialized");
	}
	&self.headers
    }

    pub fn query(&self) -> &Vec<Query> {
	if !self.initialized {
	    warn!("Request queries read not initialized");
	}
	&self.query
    }

    pub fn body(&self) -> &String {
	if !self.initialized {
	    warn!("Request body read not initialized");
	}
	&self.body
    }

    pub fn method(&self) -> &Method {
	if !self.initialized {
	    warn!("Request method read not initialized");
	}
	&self.method
    }

    pub fn path(&self) -> &String {
	if !self.initialized {
	    warn!("Request path read not initialized");
	}
	&self.path
    }

    pub fn find_header(&self, name: &str) -> Option<&Header> {
	if !self.initialized {
	    warn!("Request headers read not initialized");
	}
	self.headers.iter().find(|header| header.name().to_lowercase() == name.to_lowercase())
    }

    pub fn find_query(&self, name: &str) -> Option<&Query> {
	if !self.initialized {
	    warn!("Request queries read not initialized");
	}
	self.query.iter().find(|query| query.name() == name)
    }

    pub fn content_type(&self) -> Option<String> {
	if !self.initialized {
	    warn!("Request content type read not initialized");
	}
	let header: Option<&Header> = self.headers.iter().find(|header| header.name() == "content-type");
	match header {
	    Some(header) => Some(header.value().clone()),
	    None => None,
	}
    }

    pub fn content_length(&self) -> Option<String> {
	if !self.initialized {
	    warn!("Request content length read not initialized");
	}
	let header: Option<&Header> = self.headers.iter().find(|header| header.name() == "content-length");
	match header {
	    Some(header) => Some(header.value().clone()),
	    None => None,
	}
    }

    pub fn parse_from_str(&mut self, request: &str) {
	self.parse_request(request.to_string());
    }
    
    fn parse_request(&mut self, request: String) {
	let mut body_lines: Vec<&str> = Vec::new();
	let mut read_body: bool = false;
	for (i, line) in request.lines().enumerate() {
	    if i == 0 {
		self.parse_method_line(line);
		continue;
	    } else if line.is_empty() {
		if self.method == Method::POST || self.method == Method::PUT {
		    read_body = true;
		}
		continue;
	    }

	    if read_body {
		body_lines.push(line);
		continue;
	    } else {
		if line.contains(": ") {
		    self.parse_header_line(line);
		}
	    }
	}
	self.body = body_lines.join("\r\n");
	self.initialized = true;
    }

    fn parse_method_line(&mut self, line: &str) {
	let parts: Vec<&str> = line.split(" ").collect();
	if parts.len() != 3 {
	    error!("Invalid request line: `{}`", line);
	    return;
	}
	self.method = match parts[0] {
	    "GET" => Method::GET,
	    "POST" => Method::POST,
	    "PUT" => Method::PUT,
	    "DELETE" => Method::DELETE,
	    "HEAD" => Method::HEAD,
	    "OPTIONS" => Method::OPTIONS,
	    "CONNECT" => Method::CONNECT,
	    "TRACE" => Method::TRACE,
	    "PATCH" => Method::PATCH,
	    _ => {
		error!("Unsupported method: `{}`", parts[0]);
		return;
	    }
	};
	self.full_path = parts[1].to_string();
	self.parse_query_string(parts[1]);
    }

    fn parse_query_string(&mut self, string: &str) {
	let parts: Vec<&str> = string.split("?").collect();
	let path: &str = parts[0];
	self.path = path.to_string();
	if parts.len() == 2 {
	    let query_string: &str = parts[1];
	    let queries: Vec<&str> = query_string.split("&").collect();
	    for query in queries {
		let query_parts: Vec<&str> = query.split("=").collect();
		if query_parts.len() != 2 {
		    error!("Invalid query: `{}`", query);
		    continue;
		}
		let query: Query = Query::new(query_parts[0].to_string(), query_parts[1].to_string());
		self.query.push(query);
	    }
	};
    }

    fn parse_header_line(&mut self, line: &str) {
	let parts: Vec<&str> = line.split(": ").collect();
	if parts.len() != 2 {
	    error!("Invalid header line: `{}`", line);
	    return
	}

	if self.headers.iter().any(|header| header.name().to_lowercase() == parts[0].to_lowercase()) {
	    return;
	}
	let header: Header = Header::new(parts[0].to_string(), parts[1].to_string());
	self.headers.push(header);
    }
}

#[cfg(test)]
mod test_header {
    use super::Header;

    #[test]
    fn test_new() {
	let header: Header = Header::new("name".to_string(), "value".to_string());
	assert_eq!(header.name(), "name");
	assert_eq!(header.value(), "value");
    }
}

#[cfg(test)]
mod test_query {
    use super::Query;
    
    #[test]
    fn test_new() {
	let query: Query = Query::new("name".to_string(), "value".to_string());
	assert_eq!(query.name(), "name");
	assert_eq!(query.value(), "value");
    }
}

#[cfg(test)]
mod test_method {
    use super::Method;
	
	#[test]
    fn test_display() {
	assert_eq!(format!("{}", Method::GET), "GET");
	assert_eq!(format!("{}", Method::POST), "POST");
	assert_eq!(format!("{}", Method::PUT), "PUT");
	assert_eq!(format!("{}", Method::DELETE), "DELETE");
	assert_eq!(format!("{}", Method::HEAD), "HEAD");
	assert_eq!(format!("{}", Method::OPTIONS), "OPTIONS");
	assert_eq!(format!("{}", Method::CONNECT), "CONNECT");
	assert_eq!(format!("{}", Method::TRACE), "TRACE");
	assert_eq!(format!("{}", Method::PATCH), "PATCH");
    }
}

#[cfg(test)]
mod test_request {
    use super::{Request, Method};
    
    #[test]
    fn test_new() {
	let request: Request = Request::new();
	assert_eq!(request.headers().len(), 0);
	assert_eq!(request.query().len(), 0);
	assert_eq!(request.body(), "");
	assert_eq!(*request.method(), Method::GET);
	assert_eq!(request.path(), "");
    }

    #[test]
    fn test_parse_from_str() {
	let mut request: Request = Request::new();
	request.parse_from_str("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n");
	assert_eq!(request.headers().len(), 1);
	assert_eq!(request.headers[0].name, "Host");
	assert_eq!(request.headers[0].value, "localhost");
	assert_eq!(request.query().len(), 0);
	assert_eq!(request.body(), "");
	assert_eq!(*request.method(), Method::GET);
	assert_eq!(request.path(), "/");
    }

    #[test]
    fn test_parse_from_str_with_query() {
	let mut request: Request = Request::new();
	request.parse_from_str("GET /?name=value&test=test2 HTTP/1.1\r\nHost: localhost\r\n\r\n");
	assert_eq!(request.headers().len(), 1);
	assert_eq!(request.headers[0].name, "Host");
	assert_eq!(request.headers[0].value, "localhost");
	assert_eq!(request.query().len(), 2);
	assert_eq!(request.query[0].name, "name");
	assert_eq!(request.query[0].value, "value");
	assert_eq!(request.query[1].name, "test");
	assert_eq!(request.query[1].value, "test2");
	assert_eq!(request.body(), "");
	assert_eq!(*request.method(), Method::GET);
	assert_eq!(request.path(), "/");
    }

    #[test]
    fn test_parse_from_str_with_query_and_body() {
	let mut request: Request = Request::new();
	request.parse_from_str("POST /?name=value HTTP/1.1\r\nHost: localhost\r\nContent-Type: plain\r\n\r\nbody");
	assert_eq!(request.headers().len(), 2);
	assert_eq!(request.headers[0].name, "Host");
	assert_eq!(request.headers[0].value, "localhost");
	assert_eq!(request.headers[1].name, "Content-Type");
	assert_eq!(request.headers[1].value, "plain");
	assert_eq!(request.query().len(), 1);
	assert_eq!(request.query[0].name, "name");
	assert_eq!(request.query[0].value, "value");
	assert_eq!(request.body(), "body");
	assert_eq!(*request.method(), Method::POST);
	assert_eq!(request.path(), "/");
    }
}
