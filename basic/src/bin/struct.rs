// Classic
struct Point {
    x: i32,
    y: i32,
}

// Tuple
struct Point3D(i32, i32, i32);

// Unit
struct Empty {}

fn main() {
    let p = Point { x: -1, y: 5 };
    println!("Point.x = {}", p.x);
    println!("Point.y = {}", p.y);

    let p = Point3D(1, 2, 3);
    println!("Point3D.0 = {}", p.0);
    println!("Point3D.1 = {}", p.1);
    println!("Point3D.2 = {}", p.2);
}