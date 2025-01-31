#![allow(unused)]

// Closure as function output
fn fn_out() -> impl Fn(u32) -> u32 {
    |x| x + 1
}

fn fn_out_move() -> impl Fn() {
    let s = "hello".to_string();
    // Must move ownership into closure
    move || {
        println!("fn_out_move: {s}")
        // Cannot return s (s is owned by this closure)
        // and this closure can be called more than once
        // s
        // Can return s by cloning
        // s.clone()
    }
}

fn fn_mut_out() -> impl FnMut() {
    let mut s = "hello".to_string();
    move || {
        s += "🦀";
        println!("fn_mut_out: {}", s);
        // Cannot return s (s is owned by this closure)
        // and this closure can be called more than once
        // s
        // Can return s by cloning
        // s.clone()
    }
}

fn fn_mut_return_copy() -> impl FnMut() -> u32 {
    let mut x = 0;
    move || {
        x += 1;
        // x is copied so we can return it
        x
    }
}

fn fn_once_out() -> impl FnOnce() -> String {
    let s = "hello".to_string();
    move || {
        println!("fn_once_out: {}", s);
        // Can return s because this closure can only be called once
        s
    }
}

fn main() {
    // Fn
    let f = fn_out();
    // Call more than once
    let z = f(1);
    println!("main: z = {z}");
    let z = f(2);
    println!("main: z = {z}");

    // Fn move
    let f = fn_out_move();
    f();
    f();

    // FnMut
    let mut f = fn_mut_out();
    f();
    f();

    // FnMut return value
    let mut f = fn_mut_return_copy();
    let z = f();
    println!("z: {z}");
    let z = f();
    println!("z: {z}");
    let z = f();
    println!("z: {z}");

    let f = fn_once_out();
    let s = f();
    println!("main: {s}");
    // Cannot call twice
    // f();
}
