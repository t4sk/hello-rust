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

    // k = 0 | ax = 1 mod 2           --> x = 1
    // k = 1 | ax(2 - ax) = 1 mod 2^2 --> x = x*(2 - a*x)
    // ...
    let mut x: u128 = 1;
    for _ in 0..7 {
        // TODO: need to mod 2^k for each iteration?
        // Calculates inverse for 2^(2k)
        // x *= 2 - a * x;
        x = x.wrapping_mul(2u128.wrapping_sub(a.wrapping_mul(x)));
    }
    x
}
