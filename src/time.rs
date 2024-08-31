use std::time::{SystemTime, UNIX_EPOCH};

pub const SECOND: u128 = 1000000000;

pub struct Time;

impl Time {
    pub fn get_time() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos() // Retorna o tempo em nanosegundos
    }
}
