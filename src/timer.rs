//use std::time::{Duration, SystemTime};
use wasm_timer::{SystemTime};

pub struct Timer {
    pub delta : f64,
    current_timestamp : SystemTime,
}

impl Timer {
    pub fn new() -> Timer{ 
        let time_now = SystemTime::now();
        let millis : u128 = match time_now.elapsed() {
            Ok(elapsed) => elapsed.as_millis(),
            Err(_) => 0,
        };

        Timer {
            current_timestamp : time_now,
            delta : 0.0,
        }
    }

    pub fn update(&mut self) {
        let time_now = SystemTime::now();
        let millis : u128 = match time_now.elapsed() {
            Ok(elapsed) => elapsed.as_millis(),
            Err(_) => panic!(),
        };
        self.delta = time_now.duration_since(self.current_timestamp).unwrap().as_secs_f64();
        self.current_timestamp = time_now;
    }
}
