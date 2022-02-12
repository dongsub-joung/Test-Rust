use std::{io::{self, stdin}};

// 1316
fn main() {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    for i in buf.trim().as_bytes(){
        println!("{}", i);
    }
}