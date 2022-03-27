use std::io::{self, stdin};
fn main(){
    let n= 9;
    let list= inputing_element();
    let mut max= 0_usize;
    let mut idx= 0usize;
    for (i,j) in list.iter().enumerate(){
        if max < *j{
            max= *j;
            idx= i+1;
        }
    }
    
    println!("{} {}", idx, max);
}

fn inputing_element() -> Vec<usize>{
    let mut v: Vec<usize>= Vec::new();
    for _ in 0..9{
        let mut buf= String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let element= buf.trim().parse::<usize>().unwrap();
        v.push(element);
    }

    v
}