#![allow(unused)]

use zkstark::ff::{mul, nth_root, FieldElement};

// Function + concrete type implementation
fn modulo(a: u64, b: u64) -> u64 {
    a - a / b * b
}

// Generic implementation
use std::cmp::PartialOrd;
use std::ops::Rem;
use std::ops::{Add, Div, Mul, Sub};

fn gen_modulo<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy>(
    a: T,
    b: T,
) -> T {
    a - a / b * b
}

fn gen_modulo_2<T>(a: T, b: T) -> T
where
    T: Rem<Output = T> + Add<Output = T> + Default + PartialOrd + Copy,
{
    let z = a % b;
    if z < T::default() {
        z + b
    } else {
        z
    }
}

fn main() {
    let p: u128 = 1 + 407 * (1 << 119);
    // let x = nth_root(4);
    let x = mul(u128::MAX, u128::MAX);

    println!("{:?}", x);
}
