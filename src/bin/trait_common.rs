#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, PartialEq, Default, Eq, Hash)]
enum Color {
    #[default]
    Red,
    Green,
    Blue,
}

fn main() {
    // Default
    let p0 = Point::default();
    let p1 = Point::default();
    let p2 = Point { x: 1.0, y: 1.0 };
    let c = Color::default();

    // Clone
    let p3 = p2.clone();

    // Debug
    println!("{:?}", p0);

    // PartialEq
    println!("p0 == p1? {:?}", p0 == p1);
    println!("p0 == p2? {:?}", p0 == p2);
    println!("p2 == p3? {:?}", p2 == p3);

    let c0 = Color::Red;
    let c1 = Color::Green;
    let c2 = Color::Green;
    println!("c0 == c1? {:?}", c0 == c1);
    println!("c1 == c2? {:?}", c1 == c2);

    // Eq + Hash
    let mut map: HashMap<Color, u32> = HashMap::new();
    map.insert(Color::Red, 1);
}
