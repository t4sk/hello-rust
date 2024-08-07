#![allow(unused)]

enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn main() {
    let c = Color::Red(1);

    if let Color::Red(val) = c {
        println!("red value = {}", val);
    }

    if let Color::Blue(val) = c {
        println!("blue value = {}", val);
    }

    let num = Some(1);
    if let Some(n) = num {
        println!("num = {}", n);
    }
}
