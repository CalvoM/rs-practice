use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    println!("Reading from mock file to test functionality");
    let filename = "./poem.txt";
    let contents = fs::read_to_string(filename).expect("Error while reading file");
    println!("{}", contents);
}

struct Config{
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config{ query, filename }
}
