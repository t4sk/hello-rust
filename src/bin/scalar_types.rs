// Scalar types represent a single value
fn main() {
    // Signed integers
    // -(2**(n-1)) ~ 2**(n-1) - 1
    let i0: i8 = -1;
    let i1: i16 = 2;
    let i2: i32 = 3;
    let i3: i64 = -4;
    let i4: i128 = 5;
    // Depends on computer architecture
    let i5: isize = -6;

    // Unsigned integers
    // 0 ~ 2**n - 1
    let u0: u8 = 1;
    let u1: u16 = 2;
    let u2: u32 = 3;
    let u3: u64 = 4;
    let u4: u128 = 5;
    // Depends on computer architecture
    let u5: usize = 6;

    // Floating
    let f0: f32 = 0.01;
    let f1: f64 = 0.02;

    // Boolean
    let b: bool = true;

    // Character
    // Declared with single quote
    let c: char = 'c';
}
