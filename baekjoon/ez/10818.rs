use std::io::{self, stdin};
fn main(){
    let n= inputing();
    let list= inputing_element(n);
    let mut min= 99999_usize;
    let mut max= 0_usize;

    for i in list.iter(){
        if *i < min{
            min= *i;
        }
        if *i > max{
            max= *i;
        }
    }

    println!("{} {}", min, max);
}

fn inputing() -> (usize){
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut list= buf.split_whitespace();
    return list.next().unwrap().parse::<usize>().unwrap();
}

fn inputing_element(n: usize) -> Vec<usize>{
    let mut v: Vec<usize>= Vec::new();
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut list= buf.split_whitespace();
    
    for _ in 0..n{
        let element= list.next().unwrap().parse::<usize>().unwrap();
        v.push(element);
    }

    v
}