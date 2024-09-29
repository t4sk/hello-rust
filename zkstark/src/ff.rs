#![allow(unused)]
use std::cmp::PartialOrd;
use std::ops::Rem;
use std::ops::{Add, Div, Mul, Sub};

use crate::gcd::xgcd;

pub const P: u128 = 1 + 407 * (1 << 119);

#[derive(Debug, Clone, Copy)]
pub struct FieldElement {
    pub v: u128,
    pub p: u128,
}

impl FieldElement {
    pub fn new(v: u128, p: u128) -> Self {
        assert!(v < p, "v >= p");
        Self { v, p }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, x: &Self) -> bool {
        self.v == x.v && self.p == x.p
    }
}

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

// Generator
pub const G: FieldElement = FieldElement {
    v: 85408008396924667383611388730472331217,
    p: P,
};

pub fn nth_root(n: u128) -> FieldElement {
    // Field does not have nth root of unity where n > 2^119 or not power of two
    assert!((n <= 1 << 119) && ((n & (n - 1)) == 0));
    let mut root = G;
    let mut k: u128 = 1 << 119;
    while k != n {
        root = root * root;
        k /= 2;
    }

    root
}
