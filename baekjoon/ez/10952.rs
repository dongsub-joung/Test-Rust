use std::io::{self, stdin};
fn main(){
    let (mut a, mut b)= (9999,9999);
    while a != 0 && b != 0 {
        (a, b)= inputing();
        println!("{}", a+b);
    }
}

fn inputing() -> (usize, usize){
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut list= buf.split_whitespace();
    let a= list.next().unwrap().parse::<usize>().unwrap();
    let b= list.next().unwrap().parse::<usize>().unwrap();
    (a, b)
}