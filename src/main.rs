use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    // println!("{:?}", args);
    println!("Query: {}", query);
    println!("Filename: {}", filename);

    let content = fs::read_to_string(filename).expect("Something goes wrong");
    println!("Content: {}", content);
}
