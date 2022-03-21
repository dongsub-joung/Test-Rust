use std::io::{self, stdin};

fn main(){
    let n= inputing();
    for i in 1..n+1{
        println!("{}", i);
    }
}
fn inputing()-> usize{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    return buf.parse::<usize>().unwrap();
}