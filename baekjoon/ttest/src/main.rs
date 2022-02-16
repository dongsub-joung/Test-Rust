use std::io::{self, stdin};

fn main(){
    let (x1, y1, z1)= inputing();
    let (x2, y2, z2)= inputing();
    let (x3, y3, z3)= inputing();

}

fn inputing() -> (u32, u32, u32){
    let (mut x, mut y, mut z)= (0, 0, 0);
    
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut buf= buf.split_whitespace();
    let a= buf.next().unwrap();
    let b= buf.next().unwrap();
    let c= buf.next().unwrap();
    
    x= a.parse().unwrap();
    y= b.parse().unwrap();
    z= c.parse().unwrap();

    (x, y, z)
}