use std::{io::{self, Read}, ops::Index};

fn get_numbers() -> (usize, usize){
    let mut buf= String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut list= buf.split_whitespace();

    let num1= list.next().unwrap();
    let n= num1.trim().parse().unwrap();
    let num2= list.next().unwrap();
    let k= num2.trim().parse().unwrap();
    
    (n, k)
}

fn get_list() -> [u32; 1000] {
    let mut buf= String::new();
    let mut list= [0; 1000];

    io::stdin().read_to_string(&mut buf).unwrap();
    let mut list_strs= buf.split_whitespace();
    for (index, element) in list_strs.enumerate(){
        list[index]= element.trim().parse().unwrap();
    }

    list
}

fn main() {
    let (n ,k) = get_numbers();
    let mut result= 0;

    let mut list= get_list();

    let key: u32= list[k-1];

    for i in 0..n{
        if list[i] >= key && list[i] >0 {
            result += 1;
        }
    }

    println!("{}", result);
}
