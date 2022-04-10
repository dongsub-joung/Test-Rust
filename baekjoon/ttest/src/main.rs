use std::io::{self, stdin};
fn main(){
    let mut zero= vec![1,0];
    let mut one= vec![0,1];
    let mut t: usize;
    let mut n: usize;

    let t= inputing_T();
    for _ in 0..t+1{
        let n= inputing();
        
        for j in 2..n{
            zero[j]= zero[j-1] + zero[j-2];
            one[j]= one[j-1] + one[j-2];
        }

        println!("{} {}", zero[n], one[n]);
    }
}

fn inputing() -> usize{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf);
    let a= buf.trim().parse::<usize>().unwrap();
    return a;
}

fn inputing_T()-> usize{
    let mut buf=String::new();
    io::stdin().read_line(&mut buf);
    let a: usize= buf.parse().unwrap();
    return a;
}