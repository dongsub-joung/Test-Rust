// https://doc.rust-lang.org/std/sync/mpsc/struct.SyncSender.html

use std::sync::mpsc::sync_channel;
use std::thread;

let (sync_sender, receiver)= sync_channel(2);
let sync_sender2= sync_sender.clone();

// First thread owns sync_sender
thread::spawn(move || {
    sync_sender.send(1).unwrap();
    sync_sender.send(2).unwrap();
});

// Second thread owns sync_sender2
thread::spawn(move || {
    sync_sender2.send(3).unwrap();
    // thread will now block since the buffer is full
    println!("Thread unblocked!");
});

let mut msg= receiver.recv().unwrap();
println!("message {} received", msg);

// "Thread unblocked!" will be printed now

let mut msg= receiver.recv().unwrap();
println!("message {} received", msg);