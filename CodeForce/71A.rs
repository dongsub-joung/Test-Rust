use std::io::{self, Read};

fn limit() -> u8{
    let mut buff= String::new();
    
    io::stdin().read_line(&mut buff).expect("ERR");

    let num= buff.trim().parse().expect("ERR");

    num
}

fn word() -> (usize, String) {
    let mut buf= String::new();
    io::stdin().read_to_string(&mut buf).expect("msg");
    ( buf.len(), buf )
}

fn init(len: usize, word: String){
    let list= word.as_bytes();
    let a= list[0].to_string();
    let b= list[list.len()].to_string();
    
    println!("{}{}{}", a, len, b);
}

fn main(){
    let mut num= limit();
    if num <= 100{
        loop{
            let (len, word)= word();
            init(len, word);
            num -= 1;
            if num == 0 { break; }
        }
    } else {
        panic!("OUT RANGE")
    }
}