#![allow(unused)]

// Euclidean algorithm
// gdc(a, b) = gcd(b, a mod b) (TODO: why?)
// example
// gcd(180, 196) = gcd(16, 180) = gcd(4, 16) = 4
fn gcd_rec(a: i64, b: i64) -> i64 {
    // gcd(a, 0) = a
    if b == 0 {
        return a;
    }
    // a = bq + r
    // gcd(a, b) = gcd(b, r)
    gcd_rec(b, a.rem_euclid(b))
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a - (a / b) * b);
    }
    a
}

// Extended Euclidean algorithm
// ax + by = gcd(a, b)
// returns (x, y, gcd(a, b))

// mod inverse
// ax = 1 mod b
// ax = 1 + by
// ax - by = 1

// recursive algorithm
// gcd(a, b) = gcd(b, a mod b)
// ax + by = gcd(a, b)
//         = gcd(b, a mod b)
//         = bx1 + (a mod b)y1 = bx1 + (a - floor(a/b)b)y1
//                             = ay1 + b(x1 - floor(a/b)y1)
// x = y1
// y = x1 - floor(a/b)*y1

fn xgcd_rec(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        // Base case: gcd(a, 0) = a, and the coefficients are (1, 0)
        (a, 1, 0)
    } else {
        // Recursive case: compute gcd(b, a mod b)
        let (g, x1, y1) = xgcd_rec(b, a - (a / b) * b);
        // Update the coefficients x and y
        let x = y1;
        let y = x1 - (a / b) * y1;
        // ax + by = g
        (g, x, y)
    }
}

fn xgcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let (mut r0, mut r1) = (a, b);
    let (mut x0, mut x1) = (1, 0);
    let (mut y0, mut y1) = (0, 1);

    // TODO: why gcd(a, b) = gcd(b, a % b)

    // r0 = a, r1 = b
    // ax0 + by0 = r0 = r1q1 + r2, 0 <= r2 < r1
    // ax1 + by1 = r1 = r2q2 + r3, 0 <= r3 < r2
    // ax2 + by2 = r2 = r3q3 + r4, 0 <= r4 < r3

    // ax_n + by_n = r_n = r_{n+1}q_{n+1} + r_{n+2}, r_{n+2} = 0

    // r_{n+2} = r_n - r_{n+1}q_{n+1}
    // r2 = r0 - r1*q
    // r1 = r0

    // ax2 + by2 = r2 = r0 - r1q1 = ax0 + by0 - (ax1 + by1)q1
    //                            = a(x0 - x1q1) + b(y0 - y1q1)
    // x2 = x1 - x1q1
    // y2 = y0 - y1q1

    while r1 != 0 {
        let q = r0 / r1;
        // gcd(a, b) = gcd(b, a mod b)
        (r0, r1) = (r1, r0 - r1 * q);
        // tracking x0 and y0
        (x0, x1) = (x1, x0 - x1 * q);
        (y0, y1) = (y1, y0 - y1 * q);
    }

    // x0, y0, gcd(a, b)
    (x0, y0, r0)
}

fn main() {
    let a = 56;
    let b = 98;
    let (g, x, y) = xgcd(a, b);
    println!("xgcd({a}, {b}) = {g}");
    println!("{}*{} + {}*{} = {}", a, x, b, y, g);

    println!("gcd: {:?}", gcd(a, b));
    println!("gcd rec: {:?}", gcd_rec(a, b));
}
