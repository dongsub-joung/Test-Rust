use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32{
    println!("calculatiing slowly");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    
}