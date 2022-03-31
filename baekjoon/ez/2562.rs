use std::io::{self, stdin};
fn main(){
    let n= 10;
    let list= inputing_element();

    for i in list.iter().enumerate(){
        let pivot= i;
    }
    
    println!("{} {}", idx, max);
}

fn inputing_element() -> Vec<usize>{
    let mut v: Vec<usize>= Vec::new();
    for _ in 0..10{
        let mut buf= String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let element= buf.trim().parse::<usize>().unwrap();
        let element= element % 42;
        v.push(element);
    }

    v
}