use std::sync::Arc;
use std::thread;

let five= Arc::new(5);

for _ in 0..10{
    let five= Arc::clone(&five);
}