
use std::io::{self, stdin};

fn main(){
    let mut buf= String::new();
    stdin().read_line(&mut buf).unwrap();
    if buf.len() > 100 {
        panic!("1 to 100")
    }

    if buf.len() > 100 {
        panic!("1 to 100")
    }

    let list= buf.as_bytes();
    let mut cnt:f64= 0.0;
    for i in list.iter(){
        let lowper_line: u8= 95;
        if i > &lowper_line{
            cnt += 1.0;
        }
    }

    let divid= (list.len() as f64 / 2.0) as f64;
    let remain= divid - cnt;
    if remain <= 0.0{
        println!("{}", buf.to_lowercase());
    } else {
        println!("{}", buf.to_uppercase());
    }
}