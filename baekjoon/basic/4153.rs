use std::io::{self, stdin};

fn main(){

    loop {
        let (mut x, mut y,mut z)= inputing();
        if x > y && x > z {
            let temp= z;
            z= x;
            x= z;
        } else if y > x && y > z{
            let temp= z; 
            z= y;
            y= z;
        } 

        if theyAreTriangle(x, y, z){
            println!("rigth");
        } else {
            println!("wrong");
        }

        if x == 0 && y == 0 && z ==0{
            break;
        } 
    }
}

fn theyAreTriangle(a: u32, b: u32, c: u32) -> bool{
    if a*a + b*b == c*c{
        return true;
    } else {
        return false;
    }
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