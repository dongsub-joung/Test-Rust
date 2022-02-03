// Abstract:
    //  A thread-safe reference-counting pointer. 'Arc' stands for 'Atomically Reference Counted'.

// Thread Safety
    //  - more expensive

// Breaking cylees with Weak
    // 'downgrade' method can be used to created a non-owning Weak Pointer.

// Cloning references  
    // a, b, and foo are all Arcs that point to the same memory location
use std::sync::Arc;
let foo= Arc::new(vec![1.0, 2.0, 3.0]);
let a= foo.clone();
let b= Arc::clone(&foo);

// Deref behavior  
    // https://doc.rust-lang.org/std/rc/index.html#examples
    // '.' method-call syntax or '::' Fully qualified syntax
use std::sync::Arc;
let my_arc= Arc::new(());
let my_weak= Arc::downgrade(&my_arc);

// EX
use std::sync::Arc;
use std::thread;

let five= Arc::new(5);

for _ in 0..10{
    let five= Arc::clone(&five);
    thread::spawn(move || {
        println!("{:?}", five)
    });
}

// Ex sharing amutable AtomicUsize
use std::sync::Arc;
use std::sync::{ AtomicUsize, Ordering };
use std::thread;

let val= Arc::new(AtomicUsize::new(5));

for _ in 0..10{
    let val= Arc::clone(&val);

    thread::spawn(move || {
        let v= val.fetch_add(1, Ordering::SeqCst);
        println!("{:?}", v);
    });
}