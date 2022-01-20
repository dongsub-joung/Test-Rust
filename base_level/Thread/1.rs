https://rinthel.github.io/rust-lang-book-ko/ch16-01-threads.html
use std::thread;
use std::time::Duration;

fn main(){
    let handle= thread::spawn(||{
        for i in 1..10{
            println!("{} the spawned trhead", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{
        println!("P main {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}