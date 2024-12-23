#![allow(unused)]

fn main() {
    // Immutable by default
    let x: i32 = -123;
    // This will not compile
    // x += 1;

    // Type can be inferred in most cases
    let x = -1;

    // Mutable
    let mut y: i32 = 456;
    y += 789;

    // Constant
    const NUM: u32 = 1;

    // Shadowing
    let x: i32 = -5;
    let x = x + 5;
    let x = true;

    // Type placeholder - compiler will infer the type
    let x: _ = true;
}
