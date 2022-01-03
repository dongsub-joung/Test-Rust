use std::io;

fn return_number() -> i8{
    let mut buff= String::new();
    
    io::stdin().read_line(&mut buff).expect("ERR");

    let num= buff.trim().parse().expect("ERR");

    num
}

fn init(num: i8){
    if(num <= 100){
        if num%2 == 0 {
            print!("YES");
        }else {
            println!("NO");
        }
    } else {
        panic!("ERr");
    }
}

fn main(){
    let mut num= return_number();
    init(num);
}