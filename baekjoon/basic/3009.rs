use std::io::{self, stdin};

fn main(){
    let (mut x1,mut y1)= inputing();
    let (mut x2,mut y2)= inputing();
    let (mut x3,mut y3)= inputing();
    let (mut x4,mut y4)= inputing();


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
    io::stdin().read_line(buf).unwrap();
    let a= buf.split_whitespace().next().unwrap();
    let b= buf.split_whitespace().next().unwrap();
    
    x= a.parse().unwrap();
    x= b.parse().unwrap();

    (x, y)
}