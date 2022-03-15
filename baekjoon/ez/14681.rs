use std::io::{self, stdin};

fn main(){
    let a= inputing();
    let b= inputing();

    if a > 0 {
        if b > 0 {
            println!("1");
        } else if b < 0{
            println!("4");
        }
    } else if a < 0 {
        if b > 0 {
            println!("2");
        } else if b < 0 {
            println!("3");
        }
    }
}

fn inputing()->i32{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: i32= buf.trim().parse().unwrap();
    n
}