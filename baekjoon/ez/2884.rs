use std::io::{self, stdin};

fn main(){
    let (mut h,mut m)= inputing();
    let mut c= inputing2();
}

fn inputing()->(usize, i32){
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut list= buf.split_whitespace();
    let h: usize= list.next().unwrap().trim().parse().unwrap();
    let m: i32= list.next().unwrap().trim().parse().unwrap();

    (h, m)
}

fn inputing2() -> usize{
    
}