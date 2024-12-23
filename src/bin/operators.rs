#[allow(unused)]

fn main() {
    let a: i32 = 1;
    let b: i32 = 2;

    let c: i32 = a + b;
    let c = a - b;

    let c = a * b;
    // Integer division rounds down
    let c = a / b;
    println!("{a} / {b} = {c}");

    // Remainder != modulo operator
    // modulo operator
    // -1 % 3 = 2
    // remainder
    // -1 % 3 = -1
    let a = -1;
    let b = 3;
    let rem = a % b;
    println!("{a} % {b} = {rem}");

    // Literals
    let c = 2i32 - 3;
    let c = 3u32 * 2;
    let c = 1.23e6;
    // 1.23 x 1000000
    println!("1.23e6 = {c}");
    // Improve readability
    let c = 1_000_000_000u32;

    // Type casting
    let a: u32 = 1;
    let b = a as f32;
    println!("b = {b}");

    // Comparisions
    let a = 1;
    let b = 2;
    let c = a == b;
    let c = a != b;
    let c = a <= b;
    let c = a < b;
    let c = a >= b;
    let c = a > b;

    // Boolean
    let c = true && false;
    let c = true || false;
    let c = !true;

    // Bitwise operators
    // 101
    let a: u8 = 5;
    // 011
    let b: u8 = 3;
    println!("a & b = {:03b}", a & b);
    println!("a | b = {:03b}", a | b);
    println!("a ^ b = {:03b}", a ^ b);
    println!("!a = {:03b}", !a);
    println!("1 << 3 = {}", 1u32 << 3);
    // 10 >> 2 = 1010 >> 2 = 10
    println!("10 >> 2 = {}", 10u32 >> 2);
}
