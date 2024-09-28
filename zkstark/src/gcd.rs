#![allow(unused)]

use std::ops::{Add, Div, Mul, Sub};

pub trait Zero: Sized + Add<Self, Output = Self> {
    fn zero() -> Self;
}

pub trait One: Sized + Add<Self, Output = Self> {
    fn one() -> Self;
}

impl Zero for u64 {
    fn zero() -> Self {
        0
    }
}

impl One for u64 {
    fn one() -> Self {
        1
    }
}

pub fn xgcd<T>(mut a: T, mut b: T) -> (T, T, T)
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Zero
        + One
        + PartialEq
        + Copy,
{
    let (mut r0, mut r1) = (a, b);
    let (mut x0, mut x1) = (T::one(), T::zero());
    let (mut y0, mut y1) = (T::zero(), T::one());

    while r1 != T::zero() {
        let q = r0 / r1;
        // gcd(a, b) = gcd(b, a mod b)
        (r0, r1) = (r1, r0 - r1 * q);
        // tracking x0 and y0
        (x0, x1) = (x1, x0 - x1 * q);
        (y0, y1) = (y1, y0 - y1 * q);
    }

    (x0, y0, r0)
}
