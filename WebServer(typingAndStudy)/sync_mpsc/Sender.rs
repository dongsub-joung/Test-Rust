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

let msg = receiver.recv().unwrap();
let msg2= receiver.recv().unwrap();

assert_eq!(3, msg+msg2);

// Implementations
use std::sync::mpsc::channel;

let (tx, rx)= channel;

tx.send(1).unwrap();

drop(rx);
// This send will fail because the receiver is gone
assert_eq(tx.send.unwrap_err().0, 1);
