#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Tuple
struct Point3d(i32, i32, i32);

// Empty
struct Empty;

// Nested
#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}

fn main() {
    // Create
    let p = Point { x: 1.0, y: 1.0 };
    println!("p.x = {}", p.x);
    println!("p.y = {}", p.y);
    // Debug
    println!("{:?}", p);

    let p = Point3d(1, 2, 3);
    println!("{} {} {}", p.0, p.1, p.2);

    let empty = Empty;

    let circle = Circle {
        center: Point { x: 0.0, y: 0.0 },
        radius: 3,
    };
    println!("{:?}", circle);

    // Shortcut
    let x = 1.0;
    let y = 1.0;
    let p = Point { x, y };

    // Copy fields
    let p0 = Point { x: 1.0, y: 2.0 };
    let p1 = Point { x: 2.0, ..p0 };

    println!("{:?}", p1);

    // Update
    let mut p = Point { x: 0.0, y: 0.0 };
    p.x += 1.0;
    p.y += 1.0;
    println!("{:?}", p);
}
