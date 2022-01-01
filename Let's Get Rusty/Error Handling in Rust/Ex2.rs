// https://www.youtube.com/watch?v=wM6o70NAWUI&t=315s

// Ex 2
use std::io;
use std::io::Read;
use std::fs::{self, File};
// use std::fs::File;

fn read_usernae_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.TXT"); 
    // let mut s= String::new();
    // File::open("hello.TXT")?.read_to_string(&mut s)?;
    // Ok(s);
}

fn main(){
    
}