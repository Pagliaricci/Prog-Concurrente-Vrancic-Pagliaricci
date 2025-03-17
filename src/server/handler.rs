use std::io::{Read, Write};
use std::net::TcpStream;
mod math;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    if let Ok(_) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer);

        if let Some(i) = parse_pi_request(&request) {
            let result = math::calculate_pi(i);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nValor de Pi para {}: {}\r\n",
                i, result
            );
            stream.write_all(response.as_bytes()).unwrap();
        } else {
            let response = "HTTP/1.1 400 Bad Request\r\n\r\nFormato incorrecto\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn parse_pi_request(request: &str) -> Option<u64> {
    if request.starts_with("GET /pi/") {
        request.split_whitespace().nth(1)?.strip_prefix("/pi/")?.parse().ok()
    } else {
        None
    }
}
