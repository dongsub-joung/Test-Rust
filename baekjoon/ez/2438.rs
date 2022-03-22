use std::io::{self, stdin};

fn main(){
    let n= inputing();
    for i in 0..n+1{
        for _ in 0..i{
            print!("*");
        }
        println!("");
    }
}

fn inputing()->usize{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize= buf.trim().parse().unwrap();
    n
}