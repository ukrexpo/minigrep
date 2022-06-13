use std::env;
use std::fs;

fn main() {
    fn parse_config(args: &[String]) -> (&str, &str) {
        let query = &args[1];
        let filename = &args[2];
        (query, filename)
    }

    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let (query, filename) = parse_config(&args);
    
    println!("Query: {}", query);
    println!("Filename: {}", filename);

    let content = fs::read_to_string(filename).expect("Something goes wrong");
    println!("Content: {}", content);
}
