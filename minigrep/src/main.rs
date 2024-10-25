use std::env;
use std::fs;



fn main() {
    let args: Vec<String> = env::args().collect();
    // The first argument's beign the program's name:
    // [src/main.rs:7:5] args = [
    //  "target/debug/minigrep", ]
    let config = parse_config(&args);

    struct Config {
      query: String,
      file_path: String,
    }

    fn parse_config(args: &[String]) -> Config {
      let query = args[1].clone();
      let file_path = args[2].clone();

      Config { query, file_path }
    }

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    // adding file-reading capabilities
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");



}
