use crate::stats::SharedStats;
use crate::limiter::FileLimiter;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

pub fn handle_connection(mut stream: TcpStream, stats: SharedStats, limiter: FileLimiter) {
    let mut buffer = [0; 4096];
    if let Ok(bytes_read) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();

        if request.starts_with("GET /stats") {
            let body = stats.get_summary();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).unwrap();
        } else if request.starts_with("POST /upload") {
            let limiter = limiter.clone();
            let stats = stats.clone();

            match limiter.try_acquire() {
                Some(_permit) => {
                    if let Some(filename) = extract_filename(&request) {
                        let exception_count = count_exceptions(&request);
                        stats.add_file_stats(&filename, exception_count);
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\nProcessed file: {}",
                            18 + filename.len(),
                            filename
                        );
                        stream.write_all(response.as_bytes()).unwrap();
                    } else {
                        let response = "HTTP/1.1 400 Bad Request\r\n\r\nFile not found or empty";
                        stream.write_all(response.as_bytes()).unwrap();
                    }
                }
                None => {
                    let response = "HTTP/1.1 429 Too Many Requests\r\n\r\nToo many files being processed";
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
        } else {
            let response = "HTTP/1.1 400 Bad Request\r\n\r\nValid routes:\nPOST /upload\nGET /stats";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn extract_filename(request: &str) -> Option<String> {
    request
        .split("filename=")
        .nth(1)
        .and_then(|part| part.split_whitespace().next())
        .map(|s| s.trim_matches(['"', '\r', '\n']).to_string())
}

fn count_exceptions(request: &str) -> usize {
    request
        .lines()
        .filter(|line| line.to_lowercase().contains("exception"))
        .count()
}
