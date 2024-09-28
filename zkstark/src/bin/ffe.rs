#![allow(unused)]

use zkstark::gcd::xgcd;

#[derive(Debug, Clone, Copy)]
pub struct FieldElement {
    pub v: u64,
    pub p: u64,
}

impl FieldElement {
    pub fn new(v: u64, p: u64) -> Self {
        assert!(v < p, "v >= p");
        Self { v, p }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, x: &Self) -> bool {
        self.v == x.v && self.p == x.p
    }
}

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

// Trait
pub trait Mod<Rhs = Self> {
    type Output;
    fn modulo(self, r: Rhs) -> Self::Output;
}

impl<A, B, C> Mod<B> for A
where
    A: Rem<B, Output = C>,
    B: Clone,
    C: Add<B, Output = C> + Default + PartialOrd,
{
    type Output = C;
    fn modulo(self, r: B) -> Self::Output {
        let c = self % r.clone();
        if c < Self::Output::default() {
            c + r
        } else {
            c
        }
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, r: Self) -> Self::Output {
        assert!(self.p == r.p);

        Self {
            v: (self.v + r.v).modulo(self.p),
            p: self.p,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, r: Self) -> Self::Output {
        assert!(self.p == r.p);

        Self {
            v: (self.p + self.v - r.v).modulo(self.p),
            p: self.p,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, r: Self) -> Self::Output {
        assert!(self.p == r.p);

        Self {
            v: (self.v * r.v).modulo(self.p),
            p: self.p,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, r: Self) -> Self::Output {
        assert!(self.p == r.p);
        assert!(r.v != 0);

        // Inverse
        // (x, y, g) = xgcd(r, p)
        // rx + py = gcd(r, p) = 1
        // rx = p(-y) + 1
        // x is inverse of r mod p

        // Division
        // L / R = q
        // L = Rq mod p
        // 1 = Rx mod p
        // L = LRx mod p = R(Lx) mod p
        // q = Lx

        let (x, y, g) = xgcd(r.v, self.p);

        Self {
            v: (self.v * x).modulo(self.p),
            p: self.p,
        }
    }
}

fn main() {
    let z = modulo(1, 3);
    let z: u128 = gen_modulo(1, 3);

    let a = FieldElement::new(1, 13);
    let b = FieldElement::new(2, 13);

    assert_eq!(a, a);
    assert_ne!(a, b);
}
