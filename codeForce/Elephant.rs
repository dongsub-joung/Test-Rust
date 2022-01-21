use std::io::{self, stdin};

fn inputing() -> usize{
    let mut buff= String::new();
    stdin().read_line(&mut buff).unwrap();
    let num= buff.parse::<usize>().unwrap();
    num
}

fn main(){
    let mut x= inputing();
    let mut cnt=0;
    if x <= 5{
        cnt=1;
    } else{
        for i in 5..0{
            cnt= x/i;
            x= x%i
        }
    }
    println!("{}", cnt);
}