#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block for Rectangle.
// Everything within this impl block will be associated with the Rectangle type.
impl Rectangle {
    // &self is actually short for self: &Self.
    // Within an impl block, the type Self is an alias for the type that the
    // impl block is for.
    // If we wanted to change the instance that we've called the method on as
    // part of what the method does, we'd use &mut self as the first parameter.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
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

    println!(
        "The are of the rectangle is {} square pixels.",
        rect1.area()
    );

    // pretty print the rectangle structure
    println!("rect1 is {:#?}", rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(3);
}

// fn area1(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area2(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
