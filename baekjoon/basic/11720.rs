use std::{io::{self, stdin}};

fn main() {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: usize= buf.trim().parse().unwrap();
    
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut v= Vec::new();
    let buf= buf.trim();

    
    for i in buf.split(""){
        if i != ""{
            let j: usize = i.trim().parse().unwrap();
            v.push(j);
        }
    }
    
    // let dot= v.len() / num;
    let result: usize= v.iter().sum();

    println!("{}", result);
    
}