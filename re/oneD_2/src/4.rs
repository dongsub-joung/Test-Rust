use std::io::{self, stdin};

fn main(){
    let (a,b)= inputing();
    if b > a{
        println("<");
    } else if a > b {
        println(">");
    } else {
        println("==");
    }
}

fn inputing() -> (i32, i32) {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut list= buf.split_whitespace();
    let a: i32= list.next()..unwrap().parse().unwrap();
    let b: i32= list.next().unwrap().parse().unwrap();

    (a,b)
}