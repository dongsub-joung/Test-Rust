extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() 
{
    println!("-- Guess the number! --");
    
    println!("secret number: 0 ~ 100");
    let secret_number= rand::thread_rng().gen_range(1, 101);
    
    loop
    {
        println!("Please input your guess.");
        
        let mut guess= String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        // 32bit integer type: guess
        // Delete '\n': trim()
        // String -> integer: parse()
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        // Compare (guess, secret_number)
        match guess.cmp(&secret_number)
        {
            // cmp -> Ordering 
            // 선택지가 정해져 있고 그것을 불러서 씀.
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => 
            {
                println!("You win!");
                break;
            }
        }
    }
}
