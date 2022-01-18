use std::io;

fn intpuing() -> String{
    let mut s= String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let Y= String::from("CHAT WITH HER!");
    let N= String::from("IGNORE HIM!");
    let mut s= intpuing();

    if s.len() > 100{
        panic!("Err");
    }
    
    let mut num= 0u8;
    let mut cnt= 0u8;
    for (i,chr) in s.chars().enumerate(){
        for j in 0..i{
            if chr == s.as_bytes()[j] as char { num = num+1; }
        }
        if num == 0 { cnt = cnt+1; }
        num= 0;
    }

    if cnt%2 == 0 { println!("{}", Y) }
    else          { println!("{}", N) }
}