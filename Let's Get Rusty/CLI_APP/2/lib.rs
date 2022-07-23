use std::fs;
use std::error::Error;

pub struct Config{
    pub query: String,
    pub filename: String,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &str>{
        
    }
}