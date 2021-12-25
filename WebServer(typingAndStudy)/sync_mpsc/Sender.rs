// https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html
use std::sync::mpsc::channel;
use std::thread;

let (sender, receiver)= channel();
let sender2= sender.clone();

// First thread owns sender
thread::spawn(move || {
    sender.send(1).unwrap();
});

// Second thread owns sender2
thread::spawn(move || {
    sender2.send(2).unwrap();
})