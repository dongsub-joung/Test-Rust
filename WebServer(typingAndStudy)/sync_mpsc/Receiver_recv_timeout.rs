// https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html
struct std::sync::mpsc::Receiver
pub fn recv_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError>
// panic ex

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

let (tx, rx)= channel::<String>();

thread::spawn(move || {
    let d= Duration::from_millis(10);
    loop{
        println!("recv");
        let _r= rx.recv_timeout(d);
    }
});

thread::sleep(Duration
