use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{prelude, Read};
use std::process;

impl Config{
    fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            panic!("not enough arguments")
        }
        let query= args[1].clone();
        let filename= args[2].clone();
    
        Ok(Config{ query, filename })
    }
}
struct Config{
    query: String,
    filename: String,
}

fn run(config: Config) -> Result<(), Box<Error>>{
    let mut f= File::open(config.filename)?;

    let mut contents= String::new();
    f.read_to_string(&mut contents)?;

    println!("With test:\n{}", contents);

    Ok(())
}

fn main() {
    let args: Vec<String>= env::args().collect();
    let config= Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parshing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e)= run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }
}
