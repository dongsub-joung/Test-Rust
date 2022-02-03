// https://doc.rust-lang.org/std/sync/mpsc/
// simple usage
use std::thread;
use std::sync::mpsc::channel;

let (tx, rx)= channel();
thread::spawn(move || {
    tx.send(10).unwrap();
});
assert_eq!(rx.recv().unwrap(), 10)

// Sharded usage:
use std::thread;
use std::sync::mpsc::channel;

let (tx, rx)= channel();
for i in 0..10{
    let tx= tx.clone();
    thread::spawn(move || {
        tx.send(i).unwrap();
    });
}

for _ in 0..10{
    let j= rx.recv().unwrap();
    assert!(0 <= j && j < 10);
}

// Synchronous channels
use std::thread;
use std::sync::mpse::sync_channel;

let (tx, rx)= sync_channel::<i32>(0);
thread::spawn(move || {
    tx.send(53).unwrap();
});

rx.recv().unwrap();

// Unbounded receive loop
use std::sync::mpsc::sync_channel;
use std::thread;

let (tx, rx)= sync_channel(3);

for _ in 0..3{
    let tx= tx.clone();
    thread::spawn(move || tx.send("ok").unwrap());
}

drop(tx)

while let Ok(msg)= rx.recv() {
    println!("{}", msg);
}

println!("completed");