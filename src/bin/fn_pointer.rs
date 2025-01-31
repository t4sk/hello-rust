#![allow(unused)]

// Function pointer
fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn do_twice(f: fn(u32, u32) -> u32, x: u32, y: u32) -> u32 {
    f(x, y) + f(x, y)
}

fn push(v: &mut Vec<u32>, x: u32) {
    v.push(x);
}

fn fn_pointer_mut(f: fn(&mut Vec<u32>, u32), v: &mut Vec<u32>, x: u32) {
    f(v, x);
    f(v, x);
}

fn main() {
    // Store a function pointer to a variable
    let f: fn(u32, u32) -> u32 = add;
    println!("f: {}", f(1, 2));

    // Function pointer as input to a function
    let z = do_twice(add, 1, 2);
    println!("fn pointer: {z}");

    // Function pointer that mutates a data as input to a function
    let mut v: Vec<u32> = vec![1, 2, 3];
    fn_pointer_mut(push, &mut v, 4);
    println!("fn pointer mut: {:?}", v);
}
