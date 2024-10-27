
struct Point {
  x: i32,
  y: i32,
}
fn main() {
  let mut a = Point { x: 1, y: 2 };
  a.x += 1;
  let b = Point { y: 1, ..a };
  a.x += 1;
  println!("{}", b.x);//2
  println!("{}", a.x);//3
}
