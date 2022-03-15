use std::io::{self, stdin};

fn main(){
    let year= inputing();
    if year % 4 == 0{
        if year % 100 != 0 || year % 400 == 0{
            println!("1");
        } else {
            println!("0");
        }
    } else {
        println!("0");
    }
}

fn inputing()->usize{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize= buf.trim().parse().unwrap();
    n
}