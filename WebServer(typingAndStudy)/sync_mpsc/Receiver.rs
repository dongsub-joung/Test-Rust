// https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html
// pub struct Receiver<T> { /* fields omitted */ }
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

let (send, recv)= chaanel();

thread::spawn( move || {
    send.send("Hello world!").unwrap();
    thread::sleep(Duration::frome_secs(2));
    send.send("Delayed for 2 seconds").unwrap();
});

println!("{}", recv.recv().unwrap()); // Received immediately
println!("Waiting...");
println!("{}", recv.recv().unwrap()); // Received after 2 seconds


// Implementations impl<T> Receiver<T>

// 1.
pub fn try_recv(&self) -> Result<T, TryRecvError>

// ex
use std::sync::mpsc::{ Receiver, channel };

let (_, receiver): (_, Receiver<i32>)= channel();

assert!(receiver.try_recv().is_err());


// 2.
pub fn recv(&slef) -> Result<T, RecvError>
// ex : expected item, found keyword `let`
use std::sync::mpsc;
use std::thread;

let (send, recv)= mpsc::channel();
let handle= thread::spawn(move || {
    send.send(lu8).unwrap();
});
handle.join().unwrap();

assert_eq!(Ok(1), recv.recv());

// buffering behavior
use std::sync::mpsc;
use std::thread;
use std::sync::mpse::RecvError;

let (send, recv)= mpsc::channel();
let handle= thread::spawn( move || {
    send.send(1u8).unwrap();
    send.send(2).unwrap();
    send.send(3).unwrap();
    drop(send);
});

handle.join().unwrap();

assert_eq!(Ok(1), recv.recv());
assert_eq!(Ok(2), recv.recv());
assert_eq!(Ok(3), recv.recv());
assert_eq!(Erro(RecvError), recv.recv());
