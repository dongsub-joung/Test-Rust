use std::io::{self, stdin};

fn main(){
    let (mut h,mut m)= inputing();
    let mut c= inputing2();
    let plus_h= c / 60;
    let plus_m= c % 60;
 
    h += plus_h;
    m += plus_m;

    if m >= 60{
        m-= 60;
        h+= 1;
    } 
    if h >= 24{
        let mut h= h as i32;
        h-= 24;
    }
    println!("{} {}", h, m);
}

fn inputing()->(usize, usize){
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut list= buf.split_whitespace();
    let h: usize= list.next().unwrap().trim().parse().unwrap();
    let m: usize= list.next().unwrap().trim().parse().unwrap();

    (h, m)
}

fn inputing2() -> usize{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize= buf.trim().parse().unwrap();
    
    n
}