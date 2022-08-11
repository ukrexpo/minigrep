// Page 291

use book_minigrep::{Config, run};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Parsing arguments error: {}", err);
        process::exit(1);
    });
    
    println!("Query: {}", config.query);
    println!("Filename: {}", config.filename);

    if let Err(e) = run(config) {
        println!("Run error: {}", e);
        process::exit(1);
    };
}