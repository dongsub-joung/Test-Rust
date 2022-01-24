// Box type: 1
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T>{
        Mybox(x)
    }
}


// Deref trait
use std::ops::Deref;

impl <T> Deref for MyBox<T> {
    // associated type
    type Target= T;
    fn deref(&self) -> &T{
        &self.0
    }
}

// deref coercion
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
