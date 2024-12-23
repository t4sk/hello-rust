#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl { h: u8, s: u8, l: u8 },
}

fn main() {
    let color = Color::Blue;
    let color = Color::Rgba(100, 200, 0, 0.4);
    let color = Color::Hex(String::from("ffffff"));
    let color = Color::Hsl { h: 1, s: 2, l: 0 };
    // Debug
    println!("{:?}", color);

    // PartialEq
    println!("blue == red ? {}", Color::Blue == Color::Red);
    println!("green == green ? {}", Color::Green == Color::Green);

    // Option<T> = Some(T) | None
    let x: Option<i32> = None;
    let x: Option<i32> = Some(-1);
    println!("{:?}", x);

    // Result<T, E> = Ok(T) | Error(E)
    let res: Result<i32, &str> = Err("error 💀");
    let res: Result<i32, &str> = Ok(100);
    println!("{:?}", res);
}
