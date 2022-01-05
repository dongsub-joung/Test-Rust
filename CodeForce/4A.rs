use std::io;

fn return_number() -> u8{
    let mut buff= String::new();
    
    io::stdin().read_line(&mut buff).unwrap();

    buff.trim().parse::<u8>().unwrap();
}

fn init(num: u8){
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
    let num= return_number();
    init(num);
}