// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Speaking of different waysto write this functions,
use std::fs;
use std::io;
fn read_usename_frome_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.TXT")
}

// 'main'
use std::error:Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f= File::open("hello.TXT")?;

    Ok(())
}