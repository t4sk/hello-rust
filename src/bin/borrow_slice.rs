#![allow(unused)]

fn borrow(s: &[i32]) {
    println!("borrow {:?}", s);
}

fn borrow_mut(s: &mut [i32]) {
    s[0] = -1;
}

fn split_at(s: &[i32], i: usize) -> (&[i32], &[i32]) {
    (&s[0..i], &s[i..])
}

fn main() {
    // Slice
    // slices are always borrowed
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let s: &[i32] = &a[0..2];
    borrow(s);
    println!("slice {:?}", s);

    // Borrow mut
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    let s: &mut [i32] = &mut a[1..3];
    borrow_mut(s);
    println!("slice {:?}", s);

    // Example of function input and output
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let (s0, s1) = split_at(&a, 2);
    println!("{:?}, {:?}", s0, s1);
}
