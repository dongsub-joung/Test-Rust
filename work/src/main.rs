use std::io::{self, Read};

fn get_number() -> i16{
    let mut buf= String::new();
    io::stdin().read_to_string(&mut buf).expect("msg");
    let num= buf.trim().parse().expect("msg");
    
    num
}

fn get_win() -> i8 {
    let A: i8;
    let B: i8;
    let C: i8;
    
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
    let mut num= get_number();
    let mut result: i8;
    let mut solved: i8= 0;
    
    println!("{}", num);
    if num <= 1000{
        loop{
            result= get_win();
            
            if result >= 2 {
                solved += 1;
            }
            num -= 1;
            if num == 0 { break; }
        }

        println!("{}", solved)
    }
}
