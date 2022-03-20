use std::io::{self, stdin};

fn main(){
    let n= inputing();
    for _ in 0..n{
        let (a,b)= inputing2();
        println!("{}", a+b);
    }
}
fn inputing()-> usize{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n:usize= buf.trim().parse().unwrap();
    n
}
fn inputing2()->(usize, usize){
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut list= buf.split_whitespace();
    let a: usize= list.next().unwrap().parse().unwrap();
    let b: usize= list.next().unwrap().parse().unwrap();
    (a,b)
}