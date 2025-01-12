#![allow(unused)]

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Method for generic data type
// impl<T> tells Rust T is a generic type
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn move_to(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let mut p: Point<u32> = Point::new(1, 2);
    p.move_to(2, 3);
    println!("{:?}", p);
}
