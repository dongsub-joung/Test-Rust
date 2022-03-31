use std::io::{self, stdin};
fn main(){
    let (a,b)= inputing();
    println!("{}\n{}\n{}\n{}\n{}", a+b, a-b, a*b, a/b, a%b);
}

fn inputing() -> (usize, usize){
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut list= buf.split_whitespace();
    let a= list.next().unwrap().parse::<usize>().unwrap();
    let b= list.next().unwrap().parse::<usize>().unwrap();

    (a,b)
}