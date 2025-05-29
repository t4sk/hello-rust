#![allow(unused)]

// Scalar types represent a single value
fn main() {
    // Signed integers
    // Range: -(2^(n-1)) to 2^(n-1) - 1
    let i0: i8 = -1;
    let i1: i16 = 2;
    let i2: i32 = 3;
    let i3: i64 = -4;
    let i4: i128 = 5;
    // Depends on computer architecture
    let i5: isize = -6;

    // Unsigned integers
    // 0 to 2^n - 1
    let u0: u8 = 1;
    let u1: u16 = 2;
    let u2: u32 = 3;
    let u3: u64 = 4;
    let u4: u128 = 5;
    // Depends on computer architecture
    let u5: usize = 6;

    // Floating point numbers
    let f0: f32 = 0.01;
    let f1: f64 = 0.02;

    // Boolean
    let b: bool = true;

    // Character
    // Declared with single quote
    let c: char = 'c';

    // Type conversion
    let i: i32 = -1;
    let u: u32 = i as u32;
    println!("i32: {} to u32: {}", i, u);

    // Min and max
    let max = i32::MAX;
    let min = i32::MIN;

    // Overflow
    let mut u: u32 = u32::MAX;
    u += 1;
    // Overflow doesn't panic when compiled with --release
    println!("u32 silent overflow: {}", u);

    // Return None on overflow
    println!("u32 check overflow: {:?}", u32::checked_add(u32::MAX, 1));

    // Explicitly allow overflow
    println!("u32 allow overflow: {}", u32::wrapping_add(u32::MAX, 1));
}
