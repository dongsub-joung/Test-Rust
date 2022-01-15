use std::io::{self, Read};

fn get_number() -> u16{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num= buf.trim().parse().unwrap();
    
    num
}

fn get_win() -> u8 {
    let A: u8;
    let B: u8;
    let C: u8;
    
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    
    let mut list= buf.split_whitespace();

    let mut a= list.next().unwrap();
    A= a.trim().parse().unwrap();
    
    let mut b= list.next().unwrap();
    B= b.trim().parse().unwrap();

    let mut c= list. next().unwrap();
    C= c.trim().parse().unwrap();

    A+B+C
}

fn main() {
    let num= get_number();
    let mut result: u8= 0;
    let mut solved: u8= 0;

    if num <= 1000{
        for _ in 0..num{
            result= get_win();
            
            if result >= 2 {
                solved += 1;
            } else {
                continue;
            }
        }

        println!("{}", solved)
    }
}
