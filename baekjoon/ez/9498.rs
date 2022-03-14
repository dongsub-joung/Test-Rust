use std::io::{self, stdin};

fn main(){
    let N= inputing();
    if N >= 90 {
        println!("A");       
    } else if N >= 80 && N < 90 {
        println!("B");
    } else if N >= 70 && N < 80{
        println!("C");
    } else {
        println!("F");
    }
}

fn inputing()->usize{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize= buf.trim().parse().unwrap();
    n
}