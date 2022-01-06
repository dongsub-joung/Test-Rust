use std::io::{self, Read, BufRead};

fn limit() -> usize{
    let mut buff= String::new();
    
    let std= io::stdin();
    let mut handle= std.lock();

    handle.read_line(&mut buff).unwrap()
}

fn word() -> String {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf
}

fn init(len: usize, word: String){
    let list= word.as_str();
    let size= list.len();
    let a= &list[0..1];
    let b= &list[size-2..size-1];
    println!("{}{}{}", a, len, b);
}

fn main(){
    let mut element= String::new();
    
    let num= limit();
    if num <= 100{
        for _ in 0..num{
            element= word();
            let size= element.len() -1 ;
            
            if  size > 4{
                init(size, element);
            } else {
                println!("{}", &element);
            }
        }
    } else {
        panic!("OUT RANGE")
    }
}