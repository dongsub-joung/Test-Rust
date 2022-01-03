use std::thread;
use std::time::Duration;

// Before 
fn simulated_expensive_calculation(intensity: u32) -> u32{
    println!("calculatiing slowly");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    fn generate_workout(intensity: u32, random_number: u32){
        let expensive_closure= |num: u32| -> u32 {
            print!("calculating slowly..");
            thread::sleep(Duration::from_secs(2));
            
            num
        }

        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                simulated_expensive_calculation(intensity)
            );
            println!(
                "Next, do {} situps!",
                simulated_expensive_calculation(intensity)
            );
        } else {
            if random_number == 3{
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!"
                    , simulated_expensive_calculation(intensity)
                );
            }
        }
    }
}