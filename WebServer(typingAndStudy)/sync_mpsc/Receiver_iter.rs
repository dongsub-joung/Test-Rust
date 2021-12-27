pub fn iter(&self) -> Iter<'_, T>
// Return an iterator that will block waiting for messages.
// never panic!, reutrn None

// Ex
use std::sync::mpsc::channel;
use std::thread;

let (send, recv)= channel();

thread::spawn(move || {
    send.send(1).unwrap();
    send.send(2).unwrap();
    send.send(3).unwrap();
});

let mut iter= recv.iter();
assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(3));
assert_eq!(iter.next(), None);


pub fn try_iter(&self) -> TryIter<'_, T>
