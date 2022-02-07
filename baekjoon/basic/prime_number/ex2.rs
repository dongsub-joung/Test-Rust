use std::io::{self, stdin};

// input:  Ok
// output: Zero 
fn main(){    
    let mut all_list: Vec<usize>= Vec::new();
    let mut memo: Vec<usize>= Vec::new();

    for i in all_list{
        if prime_number(i){
            memo.push(i);
        }
    }

    let mut n= inputing();

    loop {
        let mut ctn= 0;
        if n == 0 {
            break;
        }
        for i in memo.iter(){
            if (&n<i) && (i<= &(2*n)){
                ctn+= 1;
            }
        }
        println!("{}", ctn);
        n= inputing();
    }
}


fn inputing() -> usize{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n= buf.split_whitespace().next().unwrap();
    
    n.parse::<usize>().unwrap()
}

fn prime_number(n: usize) -> bool{
    if n == 1{
        return false;
    }
    for i in 2..(((n as f64).sqrt() * 0.5 +1.0) as usize){
        if n%i == 0 {
            return false;
        }
    }
    true
}