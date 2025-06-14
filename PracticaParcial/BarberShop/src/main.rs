use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod barbershop;
use barbershop::BarberShop;

fn main() {
    // Crear la barbería con 3 sillas
    let barbershop = BarberShop::new(3);

    let barber = Arc::clone(&barbershop);
    thread::spawn(move || {
        loop {
            // El barbero trabaja siempre
            barber.unregister_customer();
            println!("Barbero está cortando el pelo...");
            thread::sleep(Duration::from_secs(3)); // Tarda 3 segundos en cortar
        }
    });

    // Simulamos clientes llegando cada cierto tiempo
    for i in 0..10 {
        let cliente = Arc::clone(&barbershop);
        thread::spawn(move || {
            println!("Cliente {} llega a la barbería", i);
            cliente.register_customer();
        });

        thread::sleep(Duration::from_secs(1)); // Llega un cliente cada 1 segundo
    }

    // Dejar que el programa siga corriendo un rato
    thread::sleep(Duration::from_secs(30));
}