mod server;


fn main() {
    let address = "127.0.0.1:3030";
    println!("Servidor escuchando en {}", address);
    server::run(address);
}
