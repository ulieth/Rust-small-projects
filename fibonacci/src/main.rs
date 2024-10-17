use std::io;

fn main() {
    loop {
        println!("Please input an integer:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input was not an integer!");
                continue;
            }
        };
        if input == 1 || input == 2 {
            println!("The {}th Fibonacci number is {}", input, 1);
            continue;
        };

        let mut fib:[u64;2] = [1,1];
        let mut index = 3;
        while index <= input {
            let temp = fib[0] + fib[1];
            fib[0] = fib[1];
            fib[1] = temp;
            index += 1
        }
        println!("The {}th Fibonacci number is {}", input, fib[1]);

    }
}
