use std::error::Error;
use std::fs::File;
use std::io::{prelude, Read};

struct Config{
    query: String,
    filename: String,
}

impl Config{
    fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("not enough arguments");
        }
        let query= args[1].clone();
        let filename= args[2].clone();
    
        Ok(Config{ query, filename })
    }
}


fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut f= File::open(config.filename)?;

    let mut contents= String::new();
    f.read_to_string(&mut contents)?;

    println!("With test:\n{}", contents);

    Ok(())
}