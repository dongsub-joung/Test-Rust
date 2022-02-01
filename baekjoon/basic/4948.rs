
use std::io::{self, stdin};

fn main(){
    let mut v: Vec<usize>= Vec::new();

    let mut num= 1_usize;
    loop {
        let mut buf= String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let n= buf.split_whitespace().next().unwrap();
        num= n.parse::<usize>().unwrap();
        
        if  num == 0{
            break;
        }
        v.push(num);
    }

    let mut result: Vec<usize>= Vec::new();
    for i in v{
        let mut vv: Vec<usize>= Vec::new();
        for n in i..2*i+1{
            if n == 2*i {
                vv.push(i);
            }
            if i % n == 0{
                break;
            }
        }
        let sum: usize= vv.iter().sum();
        result.push(sum);
    }

    for i in result{
        println!("{}", i);
    }
}