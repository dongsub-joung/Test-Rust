// https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html
// std::sync::mpsc::channel

use std::sync::mpse::channel;
use std::thread;

let (sender, receiver)= channel();

// Spawn off an expensive computation
thread::spawn(move|| {
    sender.send(expensive_computation()).unwrap();
});

// Do some useful work for awhile
// Let's see what that anser was
println!("{:?}", receiver.recv().unwrap();