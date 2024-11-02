#[allow(dead_code)]
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let point = Box::new(Point { x: 0.0, y: 0.0 });
    println!("{:?}", point);
}
