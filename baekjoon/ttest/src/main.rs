use std::io::{self, stdin};

fn main(){
    let (n, m)= inputing_number();
    
    for _ in 0..n{
        let str= inputing();
        if (m % 2) == 0 {
            if str[0] == 'B'{
                let cnt= (m / 2)+1;
                 
            } else {
                let cnt= (m / 2);
            }
        } else {
            
        }
    }
}

fn inputing_number() -> (u32, u32) {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut list= buf.split_whitespace();
    
    let n: u32= list.next().unwrap().parse().unwrap();
    let m: u32= list.next().unwrap().parse().unwrap();
    
    (n, m)
}

fn inputing() -> Vec<char>{
    let mut v: Vec<char>= Vec::new();
    
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
 
    let list=buf.chars();
    for _ in 0..buf.len(){
        let c= list.next().unwrap();
        v.push(c);
    }

    v
}