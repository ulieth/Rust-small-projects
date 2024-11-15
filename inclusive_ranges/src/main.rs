use std::time;

fn for_inclusive() {
    let mut num: u16 = 0;
    for i in 0..=(u16::MAX - 1) {
        num += i;
    }
}

fn for_exclusive() {
    let mut num: u16 = 0;
    for i in 0..u16::MAX {
        num += i;
    }
}

fn main() {
    println!("Running for 5 iterations");
    let start = time::Instant::now();
    for _ in 0..5 {
        // Uncomment the desired function to test
        for_exclusive();     // Running for 5 iterations
                             // Done: 83ns
        //for_inclusive();   // Running for 5 iterations
                             // Done: 77.125µs
    }
    let end = start.elapsed();
    println!("Done: {:?}", end);
}
