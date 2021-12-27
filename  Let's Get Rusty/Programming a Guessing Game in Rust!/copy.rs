use std::io;
use rand::Rng;
use colored::*;

fn main(){
    // Start 

    let secret_number= rand::thread_rng().gen_range(1, 101);
    let mut guess= String::new();

    println!("You guessed: {}", secret_number);

    loop{
        // inpute
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess= guess.trim().parse(){
            Ok(num) => num,
            Erro(_) => continue,
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less =>  println!("{}", "Too small!".red()),
            Ordering::Greater =>  println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
    }
}