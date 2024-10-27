#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // associated functions
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size:u32) -> Self {
        Self {
          width: size,
          height: size,
        }
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let mut r = Rectangle {
      width: 1,
      height: 2
    };
    r.set_width(2);
    Rectangle::set_width(&mut r, 2);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()

    );
    println!("rect1 is {rect1:?}");
    let square = Rectangle::square(3);
    println!("square is {square:?}");

    // the situation where a mutable reference is “downgraded” into a shared reference
    let r = &mut Box::new(Rectangle{
        width:1,
        height:2,
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1,area2);
}
