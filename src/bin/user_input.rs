use std::io;

fn main() {
    println!("Enter a number:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to readline");

    let num: u32 = input.trim().parse().expect("input not an integer");

    println!("{} x {} = {}", num, num, num * num);
}
