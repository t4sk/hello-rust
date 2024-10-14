// https://crypto.stackexchange.com/questions/47493/how-to-determine-the-multiplicative-inverse-modulo-64-or-other-power-of-two
// if ax = 1 mod 2^k then ax(2-ax) = 1 mod 2^(2k)
// if x is inverse of a mod 2^k
// then x(2 - ax) is inverse of a mod 2^(2k)

// Start x = 1, a is odd
//   |  k |  2k |
// 0 |  1 |   2 |
// 1 |  2 |   4 |
// 2 |  4 |   8 |
// 3 |  8 |  16 |
// 4 | 16 |  32 |
// 5 | 32 |  64 |
// 6 | 64 | 128 |

// Computes inverse of a, x mode r, where r = 2^128
pub fn inv(a: u128) -> u128 {
    assert!(a & 1 == 1, "a must be odd");

    // ax = 1 mod 2^k
    // ax(2 - ax) = 1 mod 2^(2k)

    // k = 0 | ax = 1 mod 2           --> x = 1
    // k = 1 | ax(2 - ax) = 1 mod 2^2 --> x = x*(2 - a*x)
    // ...
    let mut x: u128 = 1;
    for _ in 0..7 {
        // ax(2 - ax) mod 2^128 = 1 mod 2^(2k)
        //
        // ax(2 - ax) = 1 mod 2^(2k)
        // ax(2 - ax) = 2^(2k)m + 1 for m >= 0
        //
        // ax(2 - ax) = y mod 2^128 for y < 2^128
        // ax(2 - ax) = 2^128q + y for q >= 0
        //              Replace y
        //            = 2^128q + 2^(2k)n + s, n >= 0, s < 2^(2k)
        //            = 2^(2k)(2^(10 - 2k)q + n) + s
        //            m = 2^(10 - 2k)q + n so s = 1

        // Calculates inverse for 2^(2k)
        // x *= 2 - a * x;
        x = x.wrapping_mul(2u128.wrapping_sub(a.wrapping_mul(x)));
    }
    x
}

pub fn mul(x: u128, y: u128) -> (u128, u128) {
    let mask: u128 = (1 << 64) - 1;

    // (a*2^64 + b)(c*2^64 + d)
    // ac*2^128 + (ad + bc)*2^64 + bd
    // | 64 | 64 | 64 | 64
    //           |   bd   |
    //      | ad + bc |
    // |  ac   |
    let a: u128 = x >> 64;
    let b: u128 = x & mask;
    let c: u128 = y >> 64;
    let d: u128 = y & mask;

    let ac = a * c;
    let ad = a * d;
    let bc = b * c;
    let bd = b * d;

    let mid_low: u128 = (ad & mask) + (bc & mask);
    let mid_high: u128 = (ad >> 64) + (bc >> 64);
    // carry < 3 * 2**64
    let carry: u128 = mid_low + (bd >> 64);
    let high: u128 = ac + mid_high + (carry >> 64);
    let low: u128 = (mid_low << 64).wrapping_add(bd);

    (high, low)
}

pub fn add_mod(a: u128, b: u128, m: u128) -> u128 {
    assert!(a < m);
    assert!(b < m);

    let z = a.wrapping_add(b);
    if z <= a {
        // m < a + b < 2m
        // 0 < a + b - m < m
        // (a + b) - m = a - (m - b)
        a - (m - b)
    } else {
        z % m
    }
}

pub fn mul_mod(a: u128, b: u128, m: u128) -> u128 {
    assert!(a < m);
    assert!(b < m);

    let (mut high, low) = mul(a, b);

    // x*2^128 mod n = (x mod n)(2^128 mod n) mod n
    high %= m;
    for _ in 0..128 {
        high <<= 1;
        high %= m;
    }

    add_mod(high, low % m, m)
}

pub struct Montgomery {
    pub modulo: u128,
    pub inv: u128,
}

impl Montgomery {
    pub fn new(modulo: u128) -> Self {
        Self {
            modulo,
            inv: inv(modulo),
        }
    }

    // return xR mod n, where R = 2^128
    pub fn transform(&self, mut x: u128) -> u128 {
        // TODO:
        x %= self.modulo;
        // xR mod n = (x mod n)(R mod n) mod n
        let r = (1 << 64) % self.modulo;
        let (high, low) = mul(r, r);
        0
    }

    pub fn reduce(&self, u256: (u128, u128)) -> u128 {
        // TODO: modulo ops keeps algo valid?
        let (high, low) = u256;
        let q: u128 = low.wrapping_mul(self.inv);
        // TODO: fix
        // let (mut a, overflow) = high.overflowing_sub(mul(q, self.modulo).high);
        // if overflow {
        //     a += self.modulo;
        // }
        // a
        0
    }
}
