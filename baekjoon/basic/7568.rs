use std::io::{self, stdin};

fn main(){
    let num= inputing_number();
    let mut v: Vec<(u32, u32)>= Vec::new();
    let mut rank: Vec<usize>= Vec::new();
    for _ in 0..num{
        let (x1, y1)= inputing();
        v.push((x1, y1));
    }

    let mut rk=1_usize;
    for i in 0..num{
        let i= i as usize;
        for j in 0..num-1{
            let j= j as usize;
            if (v[i].0 < v[j].0) && (v[i].1 < v[j].1){
                rk+=1;
            }
        }
        rank.push(rk);
        rk= 1;
    }

    println!("{:?}", rank);
}

fn inputing_number() -> u32 {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: u32= buf.trim().parse().unwrap();
    num
}

fn inputing() -> (u32, u32){
    let (mut x, mut y)= (0, 0);
    
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut buf= buf.trim().split_whitespace();
    let a= buf.next().unwrap();
    let b= buf.next().unwrap();
    
    x= a.parse().unwrap();
    y= b.parse().unwrap();

    (x, y)
}