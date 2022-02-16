use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {}", filename);
    println!("Reading from mock file to test functionality");
    let filename = "./poem.txt";
    let contents = fs::read_to_string(filename).expect("Error while reading file");
    println!("{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
