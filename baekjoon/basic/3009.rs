use std::io::{self, stdin};

fn main(){
    let (x1, y1)= inputing();
    let (x2, y2)= inputing();
    let (x3, y3)= inputing();

    if x1 == x2 {
        println!("{}", x3);
    } else if x1 == x3{
        println!("{}", x2);
    } else {
        println!("{}", x1);
    }

    if y1 == y2 {
        println!("{}", y3);
    } else if y1 == y3 {
        println!("{}", y2);
    } else {
        println!("{}", y1)
    }
}

fn inputing() -> (i32, i32){
    let (mut x, mut y)= (0,0);
    
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut buf= buf.split_whitespace();
    let a= buf.next().unwrap();
    let b= buf.next().unwrap();
    
    x= a.parse().unwrap();
    y= b.parse().unwrap();

    (x, y)
}