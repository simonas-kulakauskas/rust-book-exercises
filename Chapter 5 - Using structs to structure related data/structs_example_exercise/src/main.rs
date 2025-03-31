#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn is_square(&self) -> bool {
        self.width == self.height
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

    let rect4 = Rectangle::square(60);

    println!(
        "The area of the rectangle is {} square pixels!",
        rect1.area()
    );

    if rect1.width() {
        // parentheses tell Rust we mean the method 'width', no parentheses means the field 'width'.
        println!("The rectangle has a nonzero width of {}.", rect1.width)
    };

    println!("Can 'rect1' hold 'rect2'? : {}", rect1.can_hold(&rect2));

    println!("Can 'rect1' hold 'rect3'? : {}", rect1.can_hold(&rect3));

    println!("rect4 is a square: {:?}", rect4);

    println!("Is rect1 a square?: {}", rect1.is_square());
    println!("Is rect4 a square?: {}", rect4.is_square());
}
