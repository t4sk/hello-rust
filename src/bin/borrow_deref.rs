#![allow(unused)]

fn borrow(s: &mut String) {
    // Dereferencing doesn't transfer ownership for &T and &mut T
    *s += "!";
}

fn main() {
    let mut x = 1;
    let r = &mut x;
    // Dereference and modify
    *r += 1;
    println!("{x}");

    let mut s = String::from("rust");
    let s1 = &mut s;
    *s1 += "!";
    println!("{s}");

    let mut s = String::from("rust");
    borrow(&mut s);
    println!("{s}");

    // Deref coercion
    let x = 1;
    let y = &x;
    let z = &x;
    // Automatically dereferenced in some situations
    let w = y + z;
    println!("{w}");
}
