#![allow(unused)]

fn main() {
    // let, immutable
    let x = 5;
    // Cannot change variables - does not compile
    // x = 6;

    // Mutable
    let mut y = 1;
    y += 1;

    println!("y = {}", y);

    // Constant
    const MY_NUM: u32 = 1;

    // Shadowing a variable
    let x: i32 = -5;
    let x = x + 5;
    let x = true;

    println!("x = {}", x);
}
