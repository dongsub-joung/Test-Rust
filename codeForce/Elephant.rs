use std::io::{self, stdin};

fn inputing() -> usize{
    let mut buff= String::new();
    stdin().read_line(&mut buff).unwrap();
    
    buff.parse::<usize>().unwrap()
}

fn main(){
    let mut x= inputing();
    let mut cnt=0;
    if x <= 5{
        cnt=1;
    } else{
        cnt= (x / 5) + 1;
    }
    println!("{}", cnt);
}