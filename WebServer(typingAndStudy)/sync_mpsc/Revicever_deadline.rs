// https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html
pub fn recv_deadline(&self, deadline: Instant) -> Result<T, RecvTimeoutError>

// EX Successfully receiving value before reaching deadline

#![feature(dealine_api)]
use std::thread;
use std::time::{ Duration, Instant };
use std::sync::mpsc;

let (send, recv)= mpsc::channel();

thread::spawn(move || {
    send.send('a').unwrap();
});

assert_eq!(
    recv.recv_deadline(Instant::now() + Duration::from_millis(400)),
    Ok('a');
)

// Receiving an error upon reaching deadline:
thread::spawn(move || {
    thread::sleep(Duration::from_millis(800));
    send.send('a').unwrap();
});