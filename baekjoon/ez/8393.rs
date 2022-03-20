use std::io::{self, stdin};

fn main(){
    let n= inputing();
    let mut sum= 0;
    for i in 0..n{
        sum += i;
    }
}
fn inputing()-> usize{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n:usize= buf.trim().parse().unwrap();
    n
}