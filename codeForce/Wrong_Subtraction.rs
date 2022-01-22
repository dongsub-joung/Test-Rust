use std::io::{self, stdin};

fn inputing()-> (usize, usize){
    let mut buf= String::new();
    stdin().read_line(&mut buf).unwrap();
    let list= buf.split_whitespace();
    let mut a= "";
    let mut b= "";
    for (i,j) in list.clone().enumerate(){
        if i == 0{
            a= j;
        } else if i == 1{
            b= j
        }
    }
    (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
}

fn main(){
    let (mut n, mut k)= inputing();
    loop {
        if n % 10 != 0{
            n-=1;
        } else{
            n= n/10;
        }
        k -= 1;
        if k==0{
            break;
        }
    }
    println!("{}", n);
}