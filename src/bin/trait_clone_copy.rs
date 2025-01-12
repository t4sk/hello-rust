#![allow(unused)]

// Clone
// - Creates a deep copy
// - Duplication might involve running arbitrary code

// Copy
// - Copy values on the stack
// - No arbitrary code execution

// A struct that implements Copy
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// This struct cannot be Copy because Vec<T> does not implement Copy
#[derive(Debug, Clone)]
struct Graph {
    vertices: Vec<u32>,
    edges: Vec<(u32, u32)>,
}

fn main() {
    // Clone
    let s0 = String::from("rust");
    let s1 = s0.clone();
    println!("s0 {:?}", s0);
    println!("s1 {:?}", s1);

    // Copy
    let x0: i32 = 1;
    // Doesn't move overship. Value is copied
    let x1 = x0;
    println!("x0 {x0}");
    println!("x1 {x1}");

    // Copy struct
    let p0 = Point { x: 1, y: 1 };
    // Values are copied, no transfer of ownership
    let p1 = p0;
    println!("p0 {:?}", p0);
    println!("p1 {:?}", p1);

    // Clone struct
    let g0 = Graph {
        vertices: vec![],
        edges: vec![],
    };
    // Must be cloned. Otherwise moves ownership
    let g1 = g0.clone();
    println!("g0 {:?}", g0);
    println!("g1 {:?}", g1);
}
