struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let square = Rectangle::square(10);

    let rect = Rectangle {
        width: 5,
        height: 3,
    };

    println!("{}", rect.area());
}
