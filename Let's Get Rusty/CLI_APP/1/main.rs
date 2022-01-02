use std::process;
use std::env;

fn main(){
    let args= env::args().collect();

    let config= Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e)= CLI_APP::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}