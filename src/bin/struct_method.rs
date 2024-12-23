#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    // Associated functions - static methods
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    // Methods
    fn dist(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let mut p = Point::new(1.0, 1.0);
    p.move_to(2.0, 0.0);

    let d = p.dist();
    println!("{}", d);
}
