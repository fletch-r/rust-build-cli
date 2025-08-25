use std::env;
use std::fs;

fn main() {
    // Using .args() instead of .args_os() as it's simpler
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // Changed from `new()` to `build()` because new should never fail
    fn build(args: &[String]) -> Config {
        if args.len() < 2 {
            panic!("Not enough arguments");
        }
        // `.clone()` is slow but for simple applications it's
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
