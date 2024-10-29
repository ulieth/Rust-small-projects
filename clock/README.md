This project implements a simple `Clock` struct in Rust that handles time calculations without dates.

## Features

- Create a clock with specific hours and minutes.
- Add and subtract minutes from the clock.
- Handles negative time input correctly.

## Implementation

Here’s how the `Clock` struct is implemented. You can create a new clock and perform operations like this:
```rust
fn main() {
    let clock = Clock::new(18, 7);
    println!("{}", clock); // Prints "18:07"

    let updated_clock = clock.add_minutes(100);
    println!("{}", updated_clock); // Prints the updated time
}
```
The Display trait is used to display the Clock's state:
```rust
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
```
You can test the clock functionality with assertions:
```rust
assert_eq!(Clock::new(18, 7), Clock::new(-54, -11513));
```
