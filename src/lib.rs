// Page 291

use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("Expected 2 arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    println!("Content: {}", content);
    Ok(())
}