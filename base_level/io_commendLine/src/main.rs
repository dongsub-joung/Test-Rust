extern crate io_commendLine;

use std::{env};
use std::process;

use io_commendLine::Config;

fn main() {
    let args: Vec<String>= env::args().collect();
    let config= Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parshing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e)= io_commendLine::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}