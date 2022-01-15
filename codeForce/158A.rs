use std::{io::{self, Read}};

fn get_numbers() -> (usize, usize){
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut list= buf.split_whitespace();

    let num1= list.next().unwrap();
    let n= num1.parse().unwrap();

    let num2= list.next().unwrap();
    let k= num2.parse().unwrap();
    
    (n, k)
}

fn get_list() -> Vec<u8> {
    let mut buf= String::new();
 
    io::stdin().read_line(&mut buf).unwrap();

    let list= buf
        .split_whitespace()
        .map(|f| f.parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();

    list
}

fn main() {
    let (n ,k) = get_numbers();
    let mut result= 0;

    let list= get_list();

    let key= list[k-1];

    for i in 0..n{
        if list[i] >= key && list[i] >0 {
            result += 1;
        }
    }

    println!("{}", result);
}
