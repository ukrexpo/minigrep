// Page 291

use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("Expected 2 arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Parsing arguments error: {}", err);
        process::exit(1);
    });
    
    println!("Query: {}", config.query);
    println!("Filename: {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let content = fs::read_to_string(config.filename).expect("Something goes wrong");
    println!("Content: {}", content);
}