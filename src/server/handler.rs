use std::io::{Read, Write};
use std::net::TcpStream;
mod math;

pub fn handle_connection(mut stream: TcpStream) {
    let mut request = String::new();
    if stream.read_to_string(&mut request).is_ok() {
        if let Some(i) = parse_pi_request(&request) {
            let result = math::calculate_pi(i).to_string();
            stream.write_all(result.as_bytes()).unwrap();
        } else {
            stream.write_all(b"Error: Formato incorrecto").unwrap();
        }
    }
}

fn parse_pi_request(request: &str) -> Option<u64> {
    request.strip_prefix("GET /pi/")?.split_whitespace().next()?.parse().ok()
}
