use std::net::TcpListener;
use std::thread;
mod handler;

pub fn run(address: &str) {
    let listener = TcpListener::bind(address).expect("No se pudo iniciar el servidor");

    for stream in listener.incoming().flatten() {
        thread::spawn(|| handler::handle_connection(stream));
    }
}
