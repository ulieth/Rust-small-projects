use std::env;
use std::fs;
use std::process;



fn main() {
    let args: Vec<String> = env::args().collect();
    // The first argument's beign the program's name:
    // [src/main.rs:7:5] args = [
    //  "target/debug/minigrep", ]
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    struct Config {
      query: String,
      file_path: String,
    }
    impl Config {
        fn build(args: &[String]) -> Result<Config, &'static str> {
          if args.len() < 3 {
            return Err("not enough arguments");
          }
          let query = args[1].clone();
          let file_path = args[2].clone();

          Ok(Config { query, file_path })
      }

    }


    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    // adding file-reading capabilities
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");



}
