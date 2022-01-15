use std::io;

fn get_n() -> u8 {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: u8= buf.trim().parse().unwrap();

    num
}

fn get_string() -> String {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf
}

fn main(){
    let mut n: u8;
    let mut x= 0;
    let s= String::new();
    
    loop {
        n = get_n();
        if n <= 150 { break; }
    }
    
    for _ in 0..n{
        let strs= get_string();
        if &strs[1..2] == "+" {
            x += 1;
        } else {
            x -= 1;
        }
    }

    println!("{}", x);
}