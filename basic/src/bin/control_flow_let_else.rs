#![allow(unused)]

fn main() {
    let x: Option<u32> = Some(1);
    let Some(u) = x else {
        // must diverge (break, return, panic!)
        panic!("x = None");
    };
    println!("u = {}", u);
}
