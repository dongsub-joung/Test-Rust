use std::io::{self, stdin};

fn main(){
    let n= inputing();
    for i in 1..10{
        println!("{} * {} = {}", n, i, n*i);
    }
}
fn inputing()-> usize{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n:usize= buf.trim().parse().unwrap();
    n
}