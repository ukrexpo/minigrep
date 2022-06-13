use std::env;
use std::fs;

fn main() {
    struct Config {
        query: String,
        filename: String,
    }
    
    fn parse_config(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config {query, filename}
    }

    let args: Vec<String> = env::args().collect();
    

    let config = parse_config(&args);
    
    println!("Query: {}", config.query);
    println!("Filename: {}", config.filename);

    let content = fs::read_to_string(config.filename).expect("Something goes wrong");
    println!("Content: {}", content);
}
