use std::env;
use std::process;
use minigrep::Config;




fn main() {
  // argument collecting
    let args: Vec<String> = env::args().collect();
  // error handling
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
