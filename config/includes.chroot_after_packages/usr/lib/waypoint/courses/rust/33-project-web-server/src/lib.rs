use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// One request from a browser, like "GET /hello?name=Sam HTTP/1.1".
#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query: Option<String>,
}

/// What the server sends back.
#[derive(Debug, PartialEq)]
pub struct Response {
    pub status: u16,
    pub content_type: String,
    pub body: String,
}

impl Response {
    /// Finished, as an example.
    pub fn new(status: u16, content_type: &str, body: &str) -> Response {
        Response { status, content_type: content_type.to_string(), body: body.to_string() }
    }

    /// Turns the response into the text sent over the network.
    pub fn to_http(&self) -> String {
        todo!()
    }
}

/// The short phrase that goes with each status code. Finished.
pub fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Bad Request",
    }
}

/// Reads "GET /hello?name=Sam HTTP/1.1". Gives back None if the line doesn't make sense.
pub fn parse_request_line(line: &str) -> Option<Request> {
    todo!()
}

/// Finds a value in a query like "name=Sam&color=blue".
pub fn query_value(query: &str, key: &str) -> Option<String> {
    todo!()
}

/// Decides what to send back for a request.
pub fn route(request: &Request) -> Response {
    todo!()
}

/// Accepts connections forever, handling each one on its own thread. Finished.
pub fn serve(listener: TcpListener) {
    for stream in listener.incoming().flatten() {
        thread::spawn(move || handle_connection(stream));
    }
}

/// Reads one request and writes one response. Finished.
fn handle_connection(mut stream: TcpStream) {
    let mut first_line = String::new();
    if BufReader::new(&stream).read_line(&mut first_line).is_err() {
        return;
    }
    let response = match parse_request_line(&first_line) {
        Some(request) => route(&request),
        None => Response::new(400, "text/plain", "That request didn't make sense."),
    };
    let _ = stream.write_all(response.to_http().as_bytes());
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    fn get(path: &str, query: Option<&str>) -> Request {
        Request { method: "GET".to_string(), path: path.to_string(), query: query.map(str::to_string) }
    }

    #[test]
    fn reading_requests() {
        assert_eq!(parse_request_line("GET /hello?name=Sam HTTP/1.1\r\n"), Some(get("/hello", Some("name=Sam"))));
        assert_eq!(parse_request_line("GET / HTTP/1.1"), Some(get("/", None)));
        assert_eq!(parse_request_line("nonsense"), None);
        assert_eq!(parse_request_line("GET hello HTTP/1.1"), None, "paths start with /");
        assert_eq!(parse_request_line("GET / FTP/1.0"), None, "the version must be HTTP");
    }

    #[test]
    fn query_values() {
        assert_eq!(query_value("name=Sam&color=blue", "color"), Some("blue".to_string()));
        assert_eq!(query_value("name=Sam", "name"), Some("Sam".to_string()));
        assert_eq!(query_value("name=Sam", "age"), None);
    }

    #[test]
    fn routing() {
        assert_eq!(route(&get("/", None)), Response::new(200, "text/html", "<h1>Welcome to my Rust server!</h1>"));
        assert_eq!(route(&get("/hello", Some("name=Ana"))).body, "Hello, Ana!");
        assert_eq!(route(&get("/hello", None)).body, "Hello, stranger!");
        assert_eq!(route(&get("/missing", None)).status, 404);
        let post = Request { method: "POST".to_string(), path: "/".to_string(), query: None };
        assert_eq!(route(&post).status, 405);
    }

    #[test]
    fn response_text() {
        assert_eq!(
            Response::new(200, "text/plain", "Hello, Sam!").to_http(),
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 11\r\nConnection: close\r\n\r\nHello, Sam!"
        );
        assert!(Response::new(404, "text/plain", "x").to_http().starts_with("HTTP/1.1 404 Not Found\r\n"));
    }

    #[test]
    fn a_real_visit() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        std::thread::spawn(move || serve(listener));

        let mut connection = std::net::TcpStream::connect(address).unwrap();
        connection.write_all(b"GET /hello?name=Waypoint HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        let mut reply = String::new();
        connection.read_to_string(&mut reply).unwrap();
        assert!(reply.starts_with("HTTP/1.1 200 OK\r\n"), "the server should answer 200 OK, but sent: {reply}");
        assert!(reply.ends_with("\r\n\r\nHello, Waypoint!"), "the server sent: {reply}");
    }
}
