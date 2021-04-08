//! minigrep
//! Written by Stefano Bieler

use std::env;
use std::fs;
fn main(){
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}\n", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong when trying to read the file.");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String
}

impl Config {

    fn new(args: &[String]) -> Config {
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        return Config { query, filename }
    }
}
