use std::io::{self, stdin};

fn main(){
    let (a,b)= inputing();
    if a > 10 && b > 10 {
        panic!("E");
    }

    println("{}", a-b);
}

fn inputing() -> (usize, usize){
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut list= buf.split_whitespace();
    let a: usize= list.next().unwrap().parse().unwrap();
    let b: usize= list.next().unwrap().parse().unwrap();

    (a,b)
}