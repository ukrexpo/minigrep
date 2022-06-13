use std::env;
use std::fs;

fn main() {
    struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        fn new(args: &[String]) -> Config {
            if args.len() < 3 {
                panic!("Expected 2 arguments");
            }
            let query = args[1].clone();
            let filename = args[2].clone();
            Config {query, filename}
        }
    }

    let args: Vec<String> = env::args().collect();
    

    let config = Config::new(&args);
    
    println!("Query: {}", config.query);
    println!("Filename: {}", config.filename);

    let content = fs::read_to_string(config.filename).expect("Something goes wrong");
    println!("Content: {}", content);
}
