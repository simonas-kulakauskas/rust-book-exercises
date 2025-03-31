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
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels!",
        rect1.area()
    );

    if rect1.width() { // parentheses tell Rust we mean the method 'width', no parentheses means the field 'width'.
        println!(
            "The rectangle has a nonzero width of {}.",
            rect1.width
        )
    };
}
