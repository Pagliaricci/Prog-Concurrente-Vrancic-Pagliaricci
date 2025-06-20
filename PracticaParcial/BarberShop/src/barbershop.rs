use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub struct BarberShop {
    chairs: Mutex<VecDeque<i64>>,
    barber_working: Condvar,
    room_capacity: usize,
}

impl BarberShop {
    pub fn new(room_capacity: usize) -> Arc<BarberShop> {
        Arc::new(BarberShop {
            chairs: Mutex::new(VecDeque::with_capacity(room_capacity)),
            barber_working: Condvar::new(),
            room_capacity,
        })
    }

    pub fn register_customer(&self){
        let mut chairs = self.chairs.lock().unwrap();
        if chairs.len() < self.room_capacity {
            println!("Cliente se va: no hay sillas disponibles");
            return;
        }
        chairs.push_back(1);
        self.barer_working.notify_one();
    }

    pub fn unregister_customer(&mut self){
        let mut chairs = self.chairs.lock().unwrap();
        while chairs.is_empty() {
            println!("Barbero duerme: no hay clientes");
            chairs = self.barber_working.wait(chairs).unwrap();
        }
        chairs.pop_front();
        println!("Barbero atendió a un cliente, quedan {} clientes", chairs.len());
    }

}

