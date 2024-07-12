// struct
struct Point<T> {
    x: T,
    y: T,
}

// enum
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// method
// impl<T> tells Rust T is generic type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let p0 = Point { x: 5, y: 10 };
    let p1 = Point { x: 1.1, y: 2.2 };
}
