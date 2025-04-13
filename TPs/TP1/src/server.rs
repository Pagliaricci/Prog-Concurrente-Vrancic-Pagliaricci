use std::net::TcpListener;
    mod handler;


    pub fn run(address: &str) {
        let listener = TcpListener::bind(address).expect("No se pudo iniciar el servidor");

        for stream in listener.incoming().flatten() {
            handler::handle_connection(stream);
        }
    }