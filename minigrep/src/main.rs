use std::env;
use std::fs;



fn main() {
    let args: Vec<String> = env::args().collect();
    // The first argument's beign the program's name:
    // [src/main.rs:7:5] args = [
    //  "target/debug/minigrep", ]
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
    // adding file-reading capabilities
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");



}
