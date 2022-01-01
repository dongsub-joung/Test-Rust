// https://www.youtube.com/watch?v=H0xBSbnQYds

use std::io;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");
    
    let secret_number: i32= rand::thread_rng().gen_range(1, 101); 

    let mut guess: String= String::new();
    println!("You guessed: {}", secret_number);

    loop{
        println!("Pleasse inpute your guess.");
    
        io::stdin(): Stdin 
            .read_line(&mut guess): Result<usize, Error>
            .expect("Failed to read line");
        
        let guess: u32= guess.trim().parse() {
            Ok(num) => num,
            Erro(_) => continue,
        } 
            
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
    }
}