use std::{io::{self, stdin}};

fn main() {
    let n= inputing();
    for _ in 0..n{
        checking();
    }
}

fn inputing() -> usize {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: usize= buf.trim().parse().unwrap();
    
    num
}

fn checking(){
    let mut v= vec![0; 1000];
    let mut group_word= 0;

    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();

    for i in buf.trim() .bytes().into_iter(){
        let i= i as usize;
        v.push(i);
    }

    let mut new_word: Vec<usize>= Vec::new();
    let mut err= 0;
    for i in v.clone(){
        if v[i] != v[i+1]{
            new_word= v[i+1..].to_vec();

            if new_word.contains(&v[i]){
                err+= 1;
            }
        }
    }

    if err == 0 {
        group_word+= 1;    
    }
    
    println!("{}", group_word);
}